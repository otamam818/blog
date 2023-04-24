<script>
  import "../globals.scss";
    import RightSide from "./RightSide.svelte";

  // TODO: Modularize Welcome Page into separate Page
  // TODO: Set up Svelte router for different pages
  // TODO: Add more content to welcome page
</script>

<div class='welcome'>
  <div class="left-side">
    <h1> Welcome </h1>
    <span> to my blog </span>
  </div>

  <RightSide />
</div>

<style lang="scss">
  .welcome {
    width: 100%;
    display: flex;
    justify-content: space-between;

    .left-side {
      padding-left: 50px;
      font-size: 6vh;

      & > span {
        transition: 200ms;
      }

      span {
        color: white;
      }
    }
  }
</style>

