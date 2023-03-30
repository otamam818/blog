<script>
  import "./globals.scss";

  const TWO_SECONDS = 2000;
  let welcomeClass = "big-text";
  setTimeout(() => {
    welcomeClass = "";
  }, TWO_SECONDS);

  // TODO: Modularize Welcome Page into separate Page
  // TODO: Set up Svelte router for different pages
  // TODO: Add more content to welcome page
</script>

<main>
  <span class={welcomeClass}> Welcome </span>
  <span> to my blog </span>
</main>

<style lang="scss">
main {
  background-color: black;
  position: fixed;
  top: 0;
  left: 0;
  min-width: 100vw;
  min-height: 100vh;
  display: flex;
  justify-content: center;
  flex-direction: column;
  align-items: center;
  z-index: 0;

  & > span {
    transition: 200ms;
  }

  span {
    color: white;
  }
}

.big-text {
  font-size: 24px;
}

</style>
