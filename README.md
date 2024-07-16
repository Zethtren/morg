# Morg

This is meant to be a markdown replacement CLI to emulate the same behavior as Emacs Org Mode and some of their extensions.
This may not be a "fully sane" conversion as I only briefly experimented with org-mode.

- [ ] TODOS.
- [ ] Agenda view (as json output)
- [ ] Tangle (In progress)
- [ ] Comments for task use tracking.

## Additional things I would like to personally add as optionals that org does not do

- [ ] syncing with github issues. (issues.md?) push and pull issues based on a file. Similar to oil.nvim as a file navigator.
- [ ] github project views.

## Tangle Mode

Tangle is meant to attach additional functionality for syncing src blocks to files.
"```language :tangle absolute_or_relative_file_path" is the syntax to use. I may add additional src variable using the `:var` syntax later on.

The src blocks will be written to the file provided. If multiple source blocks share a file name they will be appended together separated by a newline and saved to the same file.

:append may be added later but. I will want this to support languages specifically so I can use language specific comment syntax for this. You'd be better off just saving them to a different file for the time being and importing that code to where you want to use it.
