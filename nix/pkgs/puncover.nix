{ lib, python311 }:

python311.pkgs.buildPythonApplication rec {
  pname = "puncover";
  version = "0.2.2";

  src = python311.pkgs.fetchPypi {
    inherit pname version;
    hash = "sha256-AzG/7wDs5N63F+t7T3W+VMtNgL6oJP2sjGL6DdyzEEo=";
  };

  propagatedBuildInputs = with python311.pkgs; [
    flask
    jinja2
    werkzeug
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