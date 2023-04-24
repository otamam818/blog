
<div class="right-side">
  <div class="oval skinny"></div>
</div>

<style lang="scss">
  .right-side .oval.skinny {
    background-color: #4c5e8f;
    border-radius: 50%;
    rotate: 30deg;
    filter: blur(50px);
    opacity: 63%;
    position: absolute;
    animation: skinny-oval-beat 2s infinite;
  }

  @keyframes skinny-oval-beat {
    // Slow beats to mimic life into the page
    0%, 100% {
      left: 60%;
      top: 30%;
      width: 30vh;
      height: 40vh;
    }

    50% {
      left: 60%;
      top: 15%;
      width: 60vh;
      height: 80vh;
    }
  }
</style>
