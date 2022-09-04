# cloud-seed

**cloud-seed** is a very minimal alternative to cloud-init. It writes files based on directives provided in user data.

## User data format

The user data should contain `#cloud-seed` followed by a newline and then a JSON object. The only key currently defined is `files`, which should contain an array of zero or more file objects, with the structure shown below.

```json
#cloud-seed
{
  "files": [
    {
      "path": "/path/to/file/to/be/written",
      "content": "content of file, encoded according to `encoding` key",
      "encoding": "plain (default) or base64",
      "owner": "user:group",
      "permissions": "0644",
      "append": false
    }
  ]
}
```

The only required key in the file objects is `path`, which can be absolute or relative. Relative paths are intepreted relative to **cloud-seed**'s working directory

`content` defaults to an empty string.

`owner` defaults to the user running cloud-seed and their primary group, so will usually default to `root:root` (but see **Running as non-root** below).

`permissions` should be specified as an octal string and defaults to `0644`.

If `append` is `true`, the file will be appended to if it already exists, otherwise it will be truncated before the content is written.

## Supported data sources

**cloud-seed** can currently fetch user data from the metadata servers of:

* Alibaba Cloud
* AWS
* Exoscale
* GCore Labs (untested)
* Google Cloud
* Vultr

Which cloud **cloud-seed** is running in is detected automatically via DMI data.

## Running as non-root

**cloud-seed** can run as a non-root user. In this case, files can only be written at paths that this user has permission to write to. `owner` defaults to the user `cloud-seed` is running as.

## Compatibility with cloud-init

For compatibility with `cloud-init`, YAML is supported, the `#cloud-config` shebang is accepted, and `write_files` is accepted as an alias for `files`. All other `cloud-init` directives are ignored. If `cloud-init` compatibility is not required, it is recommended to use the `cloud-seed` JSON format described above.

## License

MIT or Apache 2.0, at your option.
