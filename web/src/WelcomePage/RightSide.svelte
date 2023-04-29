<script lang="ts">
  let animationOrder: String = "intro-animation";

  setTimeout(() => {
    // animationOrder = "loop-animation";
  }, 2000)
</script>

<div class="right-side">
  <div class={`oval first skinny ${animationOrder}`}></div>
</div>

<style lang="scss">
  .right-side {
    .oval {
      border-radius: 50%;
      opacity: 63%;
      position: absolute;
      filter: blur(50px);

      &.first {
        background-color: #4c5e8f;
      }

      &.intro-animation {
        animation: get-fat-inside 4s forwards;
      }

      &.loop-animation {
        animation: skinny-oval-beat 5s infinite;
      }

      &.skinny {
        // The skinny side is skinny from the animation
        rotate: 30deg;
      }
    }
  }

  @keyframes get-fat-inside {
    // intro to the oval
    0% {
      opacity: 0;
      left: 30%;
      top: 30%;
      width: 90vw;
      height: 80vh;
    }

    25%, 55% {
      opacity: 1;
      left: 60%;
      top: 15%;
      width: 80vh;
      height: 80vh;
    }

    58% {
      left: 65%;
      top: 10%;
      width: 84vh;
      height: 92vh;
    }

    100% {
      opacity: 1;
      left: 60%;
      top: 30%;
      width: 30vh;
      height: 40vh;
    }
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
