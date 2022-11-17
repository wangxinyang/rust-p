async function hello() {
  return new Promise((resolve, reject) => {
    Deno.core.print('Hello World\r\n')
    resolve('hahahaha')
  })
}

hello()
