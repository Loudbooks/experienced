<script lang="ts">
    import {onMount} from "svelte";

    export let alwaysShow: boolean = false

    let header: HTMLElement
    let h1: HTMLElement

    onMount(() => {
        if (alwaysShow) {
            header.style.opacity = '1'
        }

        onscroll = (() => {
            if (alwaysShow) return

            let innerHeight = window.innerHeight
            if (window.scrollY >= innerHeight) {
                header.style.opacity = '1'
            } else {
                header.style.opacity = '0'
            }
        })
    })

    export function setColor(color: string) {
        h1.style.color = color

        document.querySelectorAll('.header-button').forEach((button: HTMLElement) => {
            button.style.color = color
        })
    }
</script>

<container>
    <header bind:this={header}>
        <a href="/" bind:this={h1}>EXPERIENCED</a>
        <buttons>
            <a href="https://discord.com/api/oauth2/authorize?client_id=1035970092284002384&permissions=0&scope=bot%20applications.commands" class="header-button">INVITE</a>
            <a href="privacy" class="header-button">PRIVACY</a>
            <a href="terms" class="header-button">TERMS</a>
            <a href="https://github.com/sponsors/randomairborne" class="header-button">DONATE</a>
        </buttons>
    </header>
</container>

<style lang="scss">
  container {
    display: flex;
    justify-content: center;
    align-items: center;
    width: 100%;
    position: fixed;
    z-index: 1000;
  }

  header {
    transition: opacity 0.5s ease;
    display: flex;
    justify-content: space-between;
    align-items: center;
    width: 100%;
    padding: 20px 20px;
    opacity: 0;
  }

  a {
    transition: color 0.5s ease, transform 0.5s ease, opacity 0.5s ease;

    font-size: 40px;
    color: #00a1ff;
    font-family: Gabarito, sans-serif;
    font-weight: 900;
    text-decoration: none;

    &:hover {
      opacity: 0.8;
    }

    &:active {
      transform: scale(0.96);
    }

    @media (max-width: 700px) {
        font-size: 30px;
    }

    &:hover {
      color: #00a1ff;
    }
  }

  buttons {
    display: flex;
    gap: 10px;
  }

  buttons a {
    all: unset;

    transition: color 0.5s ease, transform 0.5s ease, opacity 0.5s ease;
    padding: 0 5px;
    margin-left: 10px;
    border: none;
    border-radius: 5px;
    font-family: Gabarito, sans-serif;
    font-weight: 900;
    color: #00a1ff;
    font-size: 25px;
    cursor: pointer;

    &:hover {
      opacity: 0.8;
    }

    &:active {
      transform: scale(0.96);
    }

    @media (max-width: 700px) {
        font-size: 16px;
    }
  }
</style>

