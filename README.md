# cloud-seed

**cloud-seed** is a very minimal alternative to cloud-init. It writes files based on directives provided in user data.

## User data format

The user data consists of `#cloud-seed` followed by a newline and then a JSON object. The entire user data including the `#cloud-seed` header can optionally be compressed with gzip, which is automatically detected.

The only key currently defined for the JSON object is `files`, which is an array of file objects. For example:

```json
#cloud-seed
{
  "files": [
    {
      "path": "/etc/foobar/foobar.conf",
      "content": "Zm9vYmFyCg==",
      "encoding": "base64",
      "owner": "root:root",
      "permissions": "0644",
      "append": false
    }
  ]
}
```

| Field | Description |
| --- | --- |
| `path` | Required. The path to the file to be written. Parent directories are created as needed. Relative paths are interpreted relative to **cloud-seed**'s working directory. |
| `content` | The content to be written, encoded according to the `encoding` key. Defaults to no content. |
| `encoding` | The encoding of the `content` value. Can be `plain` (the default), `base64` or `base64gzip`. |
| `owner` | The user and group that should own the file, in the format `user:group`. Defaults to the user that is running **cloud-seed** and their primary group. |
| `permissions` | The mode that files should be created with, specified as an octal string. Defaults to `0644`. If the file already exists, **cloud-seed** will not change its mode. |
| `append` | If `true`, the `content` will be appended to the file if it already exists. If `false` (the default), the file will be truncated before the content is written. |



## Supported data sources

**cloud-seed** can currently fetch user data from the metadata servers of:

* Alibaba Cloud
* AWS
* Exoscale
* GCore Labs (untested)
* Google Cloud
* Vultr

Which cloud **cloud-seed** is running in is detected automatically via DMI data.

## Compatibility with cloud-init

For compatibility with **cloud-init**, YAML is supported, the `#cloud-config` shebang is accepted, and `write_files` is accepted as an alias for `files`. The `encoding` field for each file also supports **cloud-init** values `b64`, `gz+base64`, `gzip+base64`, `gz+b64` and `gzip+b64` as aliases for the corresponding **cloud-seed** values.

All other **cloud-init** directives are ignored. If **cloud-init** compatibility is not required, it is recommended to use **cloud-seed**'s JSON format described above.

## License

MIT or Apache 2.0, at your option.
