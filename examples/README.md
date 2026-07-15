# Ansible examples

This folder contains `Ansible` examples for all endpoints in the `API`.

In the `vars` section all values written in `UPPERCASE` is set externally, but can be changed to local `hard coded` values.

For `post...` and `delete...` endpoints the `BODY` or `json_body` must be in correct `JSON` format, this can be found in the `openapi.yaml` file or start page for the `API` when it is started on your server.

For example in creating a container it can look like either one of these two examples, likewise for all other `post...` and `delete...` endpoints.

Example 1:
```json
{
  "name": "container_name",
  "image": "example.com/path/image:tag",
  "options": "--option1,--option2=value2"
}
```

Example 2:
```json
{"name": "container_name","image": "example.com/path/image:tag","options": "--option1,--option2=value2"}
```

More advanced examples will be added in the near future.