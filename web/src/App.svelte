<script>
  import Welcome from "./WelcomePage/Welcome.svelte"
</script>

<main>
  <Welcome />
</main>

<style lang="scss">
main {
  color: white;
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
}
</style>
