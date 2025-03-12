# 0.4.0

- Fixed output path building when using absolute paths for the input files
- Remove `<p>` tags from generated text
- Skip writes to the output file if the generated doc would be empty
- Support for interfaces

# 0.3.0

- fixed javadoc comments not being properly recognized, resulting in empty output files
- Added support for i18n in the generated files
- Switched to english as the default output language
- Switched to proper cli argument parsing (mostly to support the i18n feature)
