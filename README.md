# templatectl
A tool for reMarkable tablet users.  Updates the xochitl templates.json file for you programmatically so you don't need to fiddle with json.

## Usage
### Add entry
```
USAGE:
    templatectl add [FLAGS] [OPTIONS] --name <name> --filename <filename> [-icon_code "code" -c "Category1" -c "Category2" ... ]

FLAGS:
    -h, --help         Prints help information
    -l, --landscape    specify that template is landscape mode
    -V, --version      Prints version information

OPTIONS:
    -c, --category <categories>...    one or more categories to list this template under
    -f, --filename <filename>         path to png file
    -i, --icon_code <iconcode>        specify an icon to use in xochitl
    -n, --name <name>                 the name of the template
```
The add functionality will add the specified name and filename to the template list.  If you specify landscape via -l, 
it will save it as landscape (and auto-set the icon to landscape mode as well.).  If you wish to put the template into
one or more categories, specify multiple -c options to list the categories you wish to set.

### Remove entry
```
USAGE:
    templatectl remove --name <name>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --name <name>    the name of the template
```
The remove functionality will remove any templates listed in the templates.json that match the name specified in the name argument.
