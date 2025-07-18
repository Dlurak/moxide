{
  config,
  pkgs,
  lib,
  ...
}: let
  transformAttrs = attrs: keyTransform: valueTransform:
    builtins.listToAttrs (
      map (key: let
        entry = {
          key = key;
          value = attrs.${key};
        };
      in {
        name = keyTransform entry;
        value = valueTransform entry;
      }) (builtins.attrNames attrs)
    );
  yamlFormat = pkgs.formats.yaml {};
in {
  options = {
    programs.moxide.enable = lib.mkEnableOption "Enable moxide (config)";

    programs.moxide.paths = lib.mkOption {
      type = with lib.types; attrsOf str;
      default = {
        Home = "~/";
      };
      description = "Attribute-set of directories with name -> path to include in the YAML file";
    };
    programs.moxide.templates = lib.mkOption {
      default = {
        "nvim".windows = [
          {
            name = " Neovim";
            panes = ["nvim"];
          }
        ];
      };
      description = "List of directories with name and path to include in the YAML file";
    };
    programs.moxide.projects = lib.mkOption {
      default = {};
      description = "List of projects with name and path to include in the yaml files";
    };
  };

  config = let
    c = config.programs.moxide;
	pathGenerate = paths: let
		keys = builtins.attrNames paths;
		entries = map (key: let
			keyTrimmed = lib.trim key;
			value = paths.${key};
		in if  keyTrimmed == "" then value else key + ": " + value) keys;
	in lib.concatLines entries;
  in
    lib.mkIf config.programs.moxide.enable
    {
      home.file = let
        templateFileName = name: ".config/moxide/templates/${name}.yaml";
        projectTemplateFileName = name: ".config/moxide/projects/${name}.yaml";
        addName = name: attr: {inherit name;} // attr;

        directories.".config/moxide/directories.yaml".text = pathGenerate c.paths;
        templates =
          transformAttrs (c.templates)
          ({key, ...}: templateFileName key)
          ({
            key,
            value,
          }: {
            source = yamlFormat.generate (templateFileName key) (addName key value);
          });
        projects =
          transformAttrs (c.projects)
          ({key, ...}: projectTemplateFileName key)
          (
            {
              key,
              value,
            }: {
              source = yamlFormat.generate (projectTemplateFileName key) (addName key value);
            }
          );
      in
        directories // templates // projects;
    };
}
