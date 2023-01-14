# Perfin

Personal finances. This is a CLI tool (and hopefully a TUI in future) to organize personal finances. Eventually, you'll enter your finances manually but this tool helps facilitate and remove as much barriers stuff as possible.

# To-do

- [x] read transaction via command line
  - [x] append transaction to file
  - [ ] add id field to transaction (figure out best practice when it comes to PTAs)
- [ ] update transaction via command line
  - [ ] replace updated transaction into file
- [ ] remove transaction via command line
- [ ] add configuration file
  - [ ] add storage file directory
  - [ ] add custom date format
- [ ] generate summary chart for each month

# .env format

```
TRANSACTION_ID_LENGTH=<number>
JOURNAL_FILE_PATH=<path>
```
