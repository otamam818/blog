<script>
  let text;
  fetch('http://raw.githubusercontent.com/otamam818/blog/main/README.md').then(value => {
    return value.text()
  }).then(value => {
    text = value;
  });
</script>

<main>
  Hello world
  <br />
  {text ? text : ""}
</main>
