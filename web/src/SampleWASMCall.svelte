<script>
  import init, { get_form } from './pkg/md_converter.js';
  let text;
  fetch('http://raw.githubusercontent.com/otamam818/blog/main/README.md').then(value => {
    return value.text()
  }).then(value => {
    text = value;
  });

  let wasmText;
  init().then(() => {
    wasmText = get_form('# Hi there');
    console.log(wasmText);
    wasmText = get_form('Bye');
    console.log(wasmText);
  });
</script>

<main>
  Hello world
  <br />
  {text ? text : ""}
  <br />
  {wasmText ? wasmText : ""}
</main>
