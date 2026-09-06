# Does the cp-eras-ui greeter sign someone in?
#
# `custom.greetd.greeter = "cp-eras-ui"` (system/wm/hyprland.nix) runs
# `cage -s -d -- cp-eras-ui-login --greet ...` as greetd's greeter. The
# crate's own tests cover the pieces -- the greetd conversation against
# a fake greetd on a socket, the key handling, the idle frame against
# its golden -- and none of them cover the seat: cage getting a DRM
# device and the keyboard from logind as the `greeter` account, wgpu
# finding a Vulkan ICD under that account, PAM taking the password,
# greetd tearing the greeter down and starting the session. That is
# what this VM does, with a virtio GPU (no virgl, so mesa software
# rendering) standing in for the real card.
#
# The session command is pointed at a script that leaves a file in the
# user's home, because Hyprland is not the thing under test and would
# not tell us anything reliable from inside a VM. The rest of the
# module is exactly what a host gets.
#
#   nix build .#checks.x86_64-linux.greeter -L
#   nix build .#checks.x86_64-linux.greeter.driverInteractive && result/bin/nixos-test-driver
#
# It is a NixOS VM test, so it wants /dev/kvm; without it QEMU falls
# back to TCG. terra has SVM disabled in its BIOS (2026-09-06), so
# there it is the slow kind: about four minutes end to end, of which
# the greeter is drawn on the seat within ten seconds of greetd
# starting.
{ pkgs, lib, ... }:

let
  password = "hunter2";
  # The greeter draws in an era; any will do, and neomil is the one
  # terra runs.
  era = "neomil";
in
{
  name = "cp-eras-ui-greeter";

  nodes.machine =
    { pkgs, ... }:
    {
      imports = [ ../system/wm/hyprland.nix ];

      users.users.alice = {
        isNormalUser = true;
        inherit password;
      };

      custom.greetd = {
        greeter = "cp-eras-ui";
        inherit era;
        user = "alice";
        # A mark the test can wait for, then hold the session open long
        # enough to be looked at before greetd would restart the greeter.
        session = "${pkgs.writeShellScript "session" ''
          touch "$HOME/signed-in"
          sleep 60
        ''}";
      };

      # cage needs a KMS device; the default `-vga std` has none. Same
      # arrangement as nixpkgs's own cage test.
      virtualisation.qemu.options = [ "-vga none" "-device virtio-gpu-pci" ];
      virtualisation.memorySize = 2048;
      virtualisation.cores = 4;
    };

  testScript = ''
    start_all()
    machine.wait_for_unit("greetd.service")

    with subtest("the greeter comes up on the seat"):
        # `-u greeter`: the test driver runs each command through a root
        # shell whose own command line carries the pattern, so a bare
        # `pgrep -f` matches itself and proves nothing.
        machine.wait_until_succeeds("pgrep -u greeter -f 'cp-eras-ui-login --greet'", timeout=300)
        # No frame callback to wait on from outside; give a
        # software-rendered first frame time to land, then look.
        machine.sleep(30)
        machine.screenshot("greeter-idle")
        # cage and the login are both still there: no crash on the GPU
        # or the seat in the first half minute. (nixpkgs wraps cage with
        # `exec -a "$0"`, so its comm is `.cage-wrapped` and its argv[0]
        # is `.../bin/cage`: neither `-x cage` nor `-f cage-wrapped`
        # matches. The command line does.)
        machine.succeed("pgrep -u greeter -f 'bin/cage -s -d'")
        machine.succeed("pgrep -u greeter -f 'cp-eras-ui-login --greet'")

    with subtest("a wrong password is refused and the greeter stays"):
        machine.send_chars("nope\n")
        machine.sleep(15)
        machine.screenshot("greeter-denied")
        machine.succeed("pgrep -u greeter -f 'cp-eras-ui-login --greet'")
        machine.fail("test -e /home/alice/signed-in")

    with subtest("the right password starts the session"):
        # Keys typed before the window has focus are lost, and a
        # software-rendered first frame has no deadline; so type, wait,
        # and type again -- the screen takes a fresh attempt after a
        # refusal, and drops keys while one is in flight.
        for attempt in range(3):
            machine.send_chars("${password}\n")
            try:
                machine.wait_for_file("/home/alice/signed-in", timeout=90)
                break
            except Exception:
                machine.screenshot(f"greeter-attempt-{attempt}")
        else:
            raise Exception("never signed in")
        # greetd tore the greeter down: the session owns the seat now.
        machine.wait_until_fails("pgrep -u greeter -f 'cp-eras-ui-login --greet'", timeout=60)
        machine.succeed("loginctl list-sessions --no-legend | grep alice")
        machine.screenshot("session")
  '';
}
