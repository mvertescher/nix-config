{ lib,
pkgs,
buildPythonApplication,
buildPythonPackage,
fetchPypi,

# TODO: Add these back at some point
# flask,
# jinja2,
# werkzeug,

# Transitive deps
click,
itsdangerous,
babel,
markupsafe,
}:

let
# Puncover needs specific dep versions so define them below:
flask = buildPythonPackage rec {
  pname = "flask";
  version = "2.0.3";

  src = fetchPypi {
    pname = "Flask";
    inherit version;
    hash = "sha256-4RIMIoyi9VO0cN9KX6knq2YlhGdSYGmYGz6wqRkCaH0=";
  };

  propagatedBuildInputs = [
    click
    itsdangerous
    jinja2
    werkzeug
  ];

  doCheck = false;
  doInstallCheck = false;
};

jinja2 = buildPythonPackage rec {
  pname = "Jinja2";
  version = "3.0.0";

  src = fetchPypi {
    inherit pname version;
    hash = "sha256-6o192BTOnfbeanYex/HKyYr+MFuM3Eqq5OEUuNjOJMU=-";
  };

  propagatedBuildInputs = [
    babel
    markupsafe
  ];

  doCheck = false;
  doInstallCheck = false;
};

werkzeug = buildPythonPackage rec {
  pname = "werkzeug";
  version = "2.0.2";
  format = "setuptools";

  src = fetchPypi {
    pname = "Werkzeug";
    inherit version;
    hash = "sha256-qiu2/I3ujWxQTArB5/X33FgQqZA+eTtvcVqfAVva25o=";
  };

  propagatedBuildInputs = [
    markupsafe
  ];

  doCheck = false;
  doInstallCheck = false;
};

in buildPythonApplication rec {
  pname = "puncover";
  version = "0.3.4";

  src = fetchPypi {
    inherit pname version;
    hash = "sha256-1vjJ90qfzqsN3j2PndPqoo/h4C5Ei7b5jHonQqnvQKA=";
  };

  propagatedBuildInputs = [
    flask
    jinja2
    werkzeug
    pkgs.gcc-arm-embedded
  ];

  doCheck = false;
  doInstallCheck = false;

  meta = with lib; {
    homepage = "https://github.com/HBehrens/puncover";
    description = "Analyzes binaries for code size, static variables and stack usages";
    license = licenses.mit;
    maintainers = with maintainers; [ mvertescher ];
  };
}