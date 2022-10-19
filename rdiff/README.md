# rdiff
Send HTTP request by parsing command line parameters or YAML files and compare the response result and display the comparison result. 



> Command

```
rdiff add -h 

Usage: rdiff add [OPTIONS] --config <CONFIG> --profile <PROFILE>

Options:
  -c, --config <CONFIG>          Config file path
  -p, --profile <PROFILE>        Profile in file
  -e, --extra-args <EXTRA_ARGS>  for query params, use `-e key=value` for headers, use `-e %key=value` for body,, use `-e @key=value`
  -h, --help                     Print help information
```



> YAML

```request.yml
---
todo:
  request1:
    url: http://127.0.0.1:8100
    params:
      a: 100
  request2:
    url: http://127.0.0.1:8100
    params:
      c: 200
  response:
    skip_headers:
      - report
rust:
  request1:
    method: GET
    url: http://127.0.0.1:8100/req_r1
    headers:
      user-agent: Aloha
    params:
      hello: world
  request2:
    method: GET
    url: http://127.0.0.1:8100/req_r2
    params: {}
  response:
    skip_headers:
      - set-cookie
      - date



```



> Run

**rdiff add -c yaml/request.yml -p rust -e a=100**

```
1   1    | HTTP/1.1 200 OK
2   2    | content-type: "application/json; charset=utf-8"content-length: "58"
3   3    | {
4        |-  "status": "Hello, this is req1 response by rust command1"
    4    |+  "status": "Hello, this is req2 response by rust command2"
5   5    | }
```
<img width="1395" alt="image" src="https://s1.imagehub.cc/images/2022/10/19/1325a811388db5d27.png">

