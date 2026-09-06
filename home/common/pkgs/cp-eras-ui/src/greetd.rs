//! A greetd client, for the login screen run as the greeter.
//!
//! greetd speaks a four-message protocol over the unix socket it names
//! in `$GREETD_SOCK`: `create_session` for a user, then a round of
//! `auth_message` / `post_auth_message_response` until it says
//! `success`, then `start_session` with the command to hand the seat
//! to, and `cancel_session` on the way out of anything that went
//! wrong. Each message is a JSON object behind a `u32` length in
//! native byte order (greetd-ipc(7)).
//!
//! Hand-rolled rather than `greetd_ipc`: the responses are flat
//! objects of string fields, the crate would bring serde_json into a
//! tree that has neither it nor serde's derive, and the lockfile that
//! `default.nix`'s crane build vendors from would move for four
//! messages. [`read_object`] is the whole of the parser this needs and
//! [`quote`] the whole of the writer, and both are tested below.
//!
//! Everything here blocks. `screens::login` runs [`login`] on a thread
//! of its own and awaits the answer, so a slow PAM stack never holds
//! the frame.

use std::fmt;
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};

/// A password, held so that it is never printed and is wiped when it
/// is dropped.
///
/// `Debug` says `Secret(..)`; `Clone` is not derived so there is one
/// copy, taken out of the field and moved onto the greetd thread.
#[derive(Default, PartialEq, Eq)]
pub struct Secret(String);

impl Secret {
    pub fn new() -> Secret {
        Secret(String::new())
    }

    pub fn push_str(&mut self, s: &str) {
        self.0.push_str(s);
    }

    pub fn pop(&mut self) {
        self.0.pop();
    }

    pub fn clear(&mut self) {
        self.wipe();
        self.0.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// How many characters were typed -- what the field shows.
    pub fn len(&self) -> usize {
        self.0.chars().count()
    }

    /// The secret, for the one message that carries it.
    pub fn expose(&self) -> &str {
        &self.0
    }

    /// Overwrite the bytes in place. `write_volatile` so the store is
    /// not elided as dead; all-zero bytes are valid UTF-8, so the
    /// `String` stays well-formed.
    fn wipe(&mut self) {
        // SAFETY: zero bytes are valid UTF-8, so the string invariant
        // holds after the writes.
        unsafe {
            for b in self.0.as_bytes_mut() {
                std::ptr::write_volatile(b, 0);
            }
        }
    }
}

impl Drop for Secret {
    fn drop(&mut self) {
        self.wipe();
    }
}

impl fmt::Debug for Secret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Secret(..)")
    }
}

/// Why a sign-in did not happen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Refusal {
    /// greetd said no -- a wrong password, a locked account -- in its
    /// own words.
    Denied(String),
    /// The socket, the framing, or the conversation went wrong before
    /// greetd could say either way.
    Broken(String),
}

impl fmt::Display for Refusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Refusal::Denied(why) => write!(f, "greetd refused: {why}"),
            Refusal::Broken(why) => write!(f, "greetd: {why}"),
        }
    }
}

/// `$GREETD_SOCK`, which greetd sets in the greeter's environment.
pub fn socket() -> Option<PathBuf> {
    std::env::var_os("GREETD_SOCK").map(PathBuf::from)
}

/// One attempt: open a session for `user`, answer its secret prompt
/// with `secret`, and on success ask greetd to start `cmd` in it.
///
/// One connection per attempt, because greetd allows one session per
/// connection and a refused session is cancelled before this returns,
/// so the next attempt begins from nothing.
///
/// Prompts that are not for a secret (`info`, `error`, `visible`) are
/// answered with nothing; a *second* secret prompt is refused rather
/// than answered again, since the one secret has been spent and a
/// PAM stack that asks twice is not one this screen can serve.
pub fn login(sock: &Path, user: &str, secret: &Secret, cmd: &[String]) -> Result<(), Refusal> {
    let mut stream = UnixStream::connect(sock)
        .map_err(|e| Refusal::Broken(format!("cannot open {}: {e}", sock.display())))?;
    let outcome = converse(&mut stream, user, secret, cmd);
    if outcome.is_err() {
        // Best effort: the error being reported is the interesting one.
        let _ = send(&mut stream, &request(&[("type", "cancel_session")]));
        let _ = receive(&mut stream);
    }
    outcome
}

fn converse(
    stream: &mut UnixStream,
    user: &str,
    secret: &Secret,
    cmd: &[String],
) -> Result<(), Refusal> {
    send(
        stream,
        &request(&[("type", "create_session"), ("username", user)]),
    )?;
    let mut answered = false;
    loop {
        let reply = receive(stream)?;
        match reply.get("type") {
            Some("auth_message") => {
                let kind = reply.get("auth_message_type").unwrap_or("");
                let response = if kind == "secret" {
                    if answered {
                        return Err(Refusal::Denied("asked for the secret twice".into()));
                    }
                    answered = true;
                    Some(secret.expose())
                } else {
                    None
                };
                let mut message = String::from("{\"type\":\"post_auth_message_response\",\"response\":");
                match response {
                    Some(text) => message.push_str(&quote(text)),
                    None => message.push_str("null"),
                }
                message.push('}');
                send(stream, &message)?;
            }
            Some("success") => {
                let mut message = String::from("{\"type\":\"start_session\",\"cmd\":[");
                for (i, arg) in cmd.iter().enumerate() {
                    if i > 0 {
                        message.push(',');
                    }
                    message.push_str(&quote(arg));
                }
                message.push_str("],\"env\":[]}");
                send(stream, &message)?;
                let reply = receive(stream)?;
                return match reply.get("type") {
                    Some("success") => Ok(()),
                    _ => Err(refusal_of(&reply)),
                };
            }
            _ => return Err(refusal_of(&reply)),
        }
    }
}

fn refusal_of(reply: &Object) -> Refusal {
    let description = reply.get("description").unwrap_or("no description").to_string();
    match (reply.get("type"), reply.get("error_type")) {
        (Some("error"), Some("auth_error")) => Refusal::Denied(description),
        (Some("error"), _) => Refusal::Broken(description),
        (Some(other), _) => Refusal::Broken(format!("unexpected reply {other:?}")),
        (None, _) => Refusal::Broken("reply without a type".into()),
    }
}

// ---------------------------------------------------------------- framing

fn send(stream: &mut UnixStream, json: &str) -> Result<(), Refusal> {
    let len = u32::try_from(json.len())
        .map_err(|_| Refusal::Broken("request too long".into()))?;
    stream
        .write_all(&len.to_ne_bytes())
        .and_then(|_| stream.write_all(json.as_bytes()))
        .and_then(|_| stream.flush())
        .map_err(|e| Refusal::Broken(format!("write: {e}")))
}

fn receive(stream: &mut UnixStream) -> Result<Object, Refusal> {
    let mut len = [0u8; 4];
    stream
        .read_exact(&mut len)
        .map_err(|e| Refusal::Broken(format!("read: {e}")))?;
    let len = u32::from_ne_bytes(len) as usize;
    if len > 1 << 20 {
        return Err(Refusal::Broken(format!("reply of {len} bytes")));
    }
    let mut body = vec![0u8; len];
    stream
        .read_exact(&mut body)
        .map_err(|e| Refusal::Broken(format!("read: {e}")))?;
    let text = String::from_utf8(body).map_err(|_| Refusal::Broken("reply is not UTF-8".into()))?;
    read_object(&text).ok_or_else(|| Refusal::Broken("reply is not a JSON object".into()))
}

/// A request whose fields are all strings.
fn request(fields: &[(&str, &str)]) -> String {
    let mut out = String::from("{");
    for (i, (key, value)) in fields.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push_str(&quote(key));
        out.push(':');
        out.push_str(&quote(value));
    }
    out.push('}');
    out
}

// ------------------------------------------------------------------- json

/// `s` as a JSON string literal.
pub fn quote(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// The string-valued fields of a flat JSON object. Fields of any other
/// type are skipped over and not kept; greetd sends none in a reply.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Object(Vec<(String, String)>);

impl Object {
    pub fn get(&self, key: &str) -> Option<&str> {
        self.0
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
    }
}

/// Parse one JSON object. `None` for anything that is not one, or is
/// malformed.
pub fn read_object(text: &str) -> Option<Object> {
    let mut p = Parser {
        s: text.as_bytes(),
        i: 0,
    };
    p.space();
    let object = p.object()?;
    p.space();
    (p.i == p.s.len()).then_some(object)
}

struct Parser<'a> {
    s: &'a [u8],
    i: usize,
}

impl Parser<'_> {
    fn peek(&self) -> Option<u8> {
        self.s.get(self.i).copied()
    }

    fn space(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\t' | b'\n' | b'\r')) {
            self.i += 1;
        }
    }

    fn eat(&mut self, c: u8) -> Option<()> {
        (self.peek() == Some(c)).then(|| self.i += 1)
    }

    fn object(&mut self) -> Option<Object> {
        self.eat(b'{')?;
        let mut fields = Vec::new();
        self.space();
        if self.eat(b'}').is_some() {
            return Some(Object(fields));
        }
        loop {
            self.space();
            let key = self.string()?;
            self.space();
            self.eat(b':')?;
            self.space();
            if let Some(value) = self.value()? {
                fields.push((key, value));
            }
            self.space();
            if self.eat(b',').is_some() {
                continue;
            }
            self.eat(b'}')?;
            return Some(Object(fields));
        }
    }

    /// One value: `Some(Some(s))` for a string, `Some(None)` for a
    /// value of any other type that was skipped, `None` on a syntax
    /// error.
    fn value(&mut self) -> Option<Option<String>> {
        match self.peek()? {
            b'"' => self.string().map(Some),
            b'{' => self.object().map(|_| None),
            b'[' => {
                self.eat(b'[')?;
                self.space();
                if self.eat(b']').is_some() {
                    return Some(None);
                }
                loop {
                    self.space();
                    self.value()?;
                    self.space();
                    if self.eat(b',').is_some() {
                        continue;
                    }
                    self.eat(b']')?;
                    return Some(None);
                }
            }
            b't' => self.word("true"),
            b'f' => self.word("false"),
            b'n' => self.word("null"),
            b'-' | b'0'..=b'9' => {
                while matches!(
                    self.peek(),
                    Some(b'-' | b'+' | b'.' | b'e' | b'E' | b'0'..=b'9')
                ) {
                    self.i += 1;
                }
                Some(None)
            }
            _ => None,
        }
    }

    fn word(&mut self, w: &str) -> Option<Option<String>> {
        let end = self.i + w.len();
        (self.s.get(self.i..end)? == w.as_bytes()).then(|| {
            self.i = end;
            None
        })
    }

    fn string(&mut self) -> Option<String> {
        self.eat(b'"')?;
        let mut out = Vec::new();
        loop {
            match self.peek()? {
                b'"' => {
                    self.i += 1;
                    return String::from_utf8(out).ok();
                }
                b'\\' => {
                    self.i += 1;
                    match self.peek()? {
                        b'"' => out.push(b'"'),
                        b'\\' => out.push(b'\\'),
                        b'/' => out.push(b'/'),
                        b'b' => out.push(0x08),
                        b'f' => out.push(0x0c),
                        b'n' => out.push(b'\n'),
                        b'r' => out.push(b'\r'),
                        b't' => out.push(b'\t'),
                        b'u' => {
                            self.i += 1;
                            let mut code = self.hex4()?;
                            // A surrogate pair spells one character.
                            if (0xd800..0xdc00).contains(&code) {
                                self.eat(b'\\')?;
                                self.eat(b'u')?;
                                let low = self.hex4()?;
                                code = 0x10000 + ((code - 0xd800) << 10) + low.checked_sub(0xdc00)?;
                            }
                            let c = char::from_u32(code)?;
                            let mut buf = [0u8; 4];
                            out.extend_from_slice(c.encode_utf8(&mut buf).as_bytes());
                            continue;
                        }
                        _ => return None,
                    }
                    self.i += 1;
                }
                c => {
                    out.push(c);
                    self.i += 1;
                }
            }
        }
    }

    /// Four hex digits at `i`.
    fn hex4(&mut self) -> Option<u32> {
        let digits = self.s.get(self.i..self.i + 4)?;
        let code = u32::from_str_radix(std::str::from_utf8(digits).ok()?, 16).ok()?;
        self.i += 4;
        Some(code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::net::UnixListener;

    #[test]
    fn quoting_round_trips_through_the_reader() {
        for s in ["", "plain", "with \"quotes\" and \\ slashes", "tab\tnl\n", "\u{1}", "héllo 日本 🔑"] {
            let json = format!("{{\"k\":{}}}", quote(s));
            let object = read_object(&json).expect(&json);
            assert_eq!(object.get("k"), Some(s), "{json}");
        }
    }

    #[test]
    fn reads_greetd_replies() {
        let reply = read_object(
            r#"{"type":"auth_message","auth_message_type":"secret","auth_message":"Password: "}"#,
        )
        .unwrap();
        assert_eq!(reply.get("type"), Some("auth_message"));
        assert_eq!(reply.get("auth_message_type"), Some("secret"));
        assert_eq!(reply.get("auth_message"), Some("Password: "));

        let reply = read_object(
            r#"{ "type" : "error", "error_type": "auth_error", "description": "pam_authenticate: \"no\"" }"#,
        )
        .unwrap();
        assert_eq!(reply.get("description"), Some("pam_authenticate: \"no\""));

        // Fields of other types are skipped, not choked on.
        let reply = read_object(r#"{"a":[1,2,{"x":null}],"b":true,"c":-1.5e3,"type":"success"}"#).unwrap();
        assert_eq!(reply.get("type"), Some("success"));
        assert_eq!(reply.get("a"), None);

        assert_eq!(read_object("\"not an object\""), None);
        assert_eq!(read_object("{\"unterminated\":\"x"), None);
        assert_eq!(read_object("{} trailing"), None);
        assert_eq!(read_object("{\"k\":\"\\uD83D\\uDD11\"}").unwrap().get("k"), Some("🔑"));
    }

    #[test]
    fn secret_is_redacted_and_counts_characters() {
        let mut s = Secret::new();
        s.push_str("pässword");
        assert_eq!(format!("{s:?}"), "Secret(..)");
        assert_eq!(s.len(), 8);
        s.pop();
        assert_eq!(s.expose(), "pässwor");
        s.clear();
        assert!(s.is_empty());
    }

    /// A fake greetd on a socket under a temp dir: it plays one side
    /// of the conversation and records the other.
    fn fake_greetd(
        script: Vec<&'static str>,
    ) -> (PathBuf, std::thread::JoinHandle<Vec<String>>) {
        let dir = std::env::temp_dir().join(format!(
            "cp-eras-ui-greetd-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let sock = dir.join("sock");
        let listener = UnixListener::bind(&sock).unwrap();
        let handle = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut seen = Vec::new();
            for reply in script {
                let request = receive(&mut stream).unwrap();
                seen.push(format!("{request:?}"));
                send(&mut stream, reply).unwrap();
            }
            // Whatever comes after the script (a cancel) is read so
            // the client is not left with a broken pipe.
            while let Ok(request) = receive(&mut stream) {
                seen.push(format!("{request:?}"));
                let _ = send(&mut stream, r#"{"type":"success"}"#);
            }
            let _ = std::fs::remove_dir_all(dir);
            seen
        });
        (sock, handle)
    }

    #[test]
    fn a_right_password_starts_the_session() {
        let (sock, greetd) = fake_greetd(vec![
            r#"{"type":"auth_message","auth_message_type":"info","auth_message":"welcome"}"#,
            r#"{"type":"auth_message","auth_message_type":"secret","auth_message":"Password: "}"#,
            r#"{"type":"success"}"#,
            r#"{"type":"success"}"#,
        ]);
        let mut secret = Secret::new();
        secret.push_str("hunter2");
        let cmd = vec!["uwsm start hyprland-uwsm.desktop".to_string()];
        assert_eq!(login(&sock, "mverte", &secret, &cmd), Ok(()));
        let seen = greetd.join().unwrap();
        assert_eq!(seen.len(), 4, "{seen:?}");
        assert!(seen[0].contains("create_session") && seen[0].contains("mverte"));
        assert!(seen[1].contains("post_auth_message_response") && !seen[1].contains("hunter2"));
        assert!(seen[2].contains("hunter2"));
        assert!(seen[3].contains("start_session"), "{}", seen[3]);
    }

    #[test]
    fn a_wrong_password_is_denied_and_the_session_cancelled() {
        let (sock, greetd) = fake_greetd(vec![
            r#"{"type":"auth_message","auth_message_type":"secret","auth_message":"Password: "}"#,
            r#"{"type":"error","error_type":"auth_error","description":"Authentication failure"}"#,
        ]);
        let mut secret = Secret::new();
        secret.push_str("wrong");
        assert_eq!(
            login(&sock, "mverte", &secret, &[]),
            Err(Refusal::Denied("Authentication failure".into()))
        );
        let seen = greetd.join().unwrap();
        assert!(seen.last().unwrap().contains("cancel_session"), "{seen:?}");
    }

    #[test]
    fn no_socket_is_broken_not_denied() {
        let secret = Secret::new();
        assert!(matches!(
            login(Path::new("/nonexistent/greetd.sock"), "u", &secret, &[]),
            Err(Refusal::Broken(_))
        ));
    }
}
