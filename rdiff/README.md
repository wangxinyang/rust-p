# rdiff
Send HTTP request by parsing command line parameters or YAML files and compare the response result and display the comparison result. 



> Command

```
rdiff -h

diff http response from two request

Usage: rdiff [OPTIONS]

Options:
  -b, --base <BASE>
          url base
  -p, --ports <PORTS>...
          url port [default: 8100]
  -m, --method <METHOD>
          http method [default: get]
  -c, --params <PARAMS>...
          http params
  -h, --help
          Print help information (use `--help` for more detail)
  -V, --version
          Print version information
```



> YAML

```request.yml
requests:
  - base: http://127.0.0.1
    port: 8000
    method: get
    param: req1
  - base: http://127.0.0.1
    port: 8000
    method: get
    param: req2


```



> Result

```
-Hello, this is req1 response by rust command1
+Hello, this is req2 response by rust command2
```
<img width="1395" alt="image" src="https://user-images.githubusercontent.com/11283532/193224470-fc3e3a1f-a445-4e9f-8d87-94aa3e846d43.png">


