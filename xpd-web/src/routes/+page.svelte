<script lang="ts">
    import {onMount} from "svelte";
    import {revealWord, viewListen} from "$lib/animations";
    import Header from "../components/Header.svelte";

    let pageHeader;

    let header: HTMLElement;
    let description: HTMLElement
    let blockTwoHeader: HTMLElement
    let blockTwoDescription: HTMLElement
    let inviteButton: HTMLElement
    let glitch: HTMLElement
    let gradient: HTMLElement
    let img: HTMLImageElement
    let blockThreeHeader: HTMLElement
    let blockFourHeader: HTMLElement

    let isNumberBlock = false;

    onMount(() => {
        let headerText = header.innerText;
        let descriptionText = description.innerText;

        setTimeout(() => {
            document.querySelectorAll('.home-button').forEach((button: HTMLElement, index) => {
                setTimeout(() => {
                    button.style.transform = 'translateY(0)';
                    button.style.opacity = '1';
                }, index * 100)
            })
        }, 300)

        revealWord(header, headerText, 0)
        revealWord(description, descriptionText, 0, 1, 6)

        viewListen(header, () => revealWord(header, headerText, 0))
        viewListen(description, () => revealWord(description, descriptionText, 0, 1, 6))
        viewListen(blockTwoHeader, () => pageHeader.setColor('#00a1ff'))
        viewListen(img, (isView) => {
            if (!isView) {
                img.style.opacity = '0';
                img.style.transform = 'translateY(20px)';
                return
            } else {
                img.style.opacity = '1';
                img.style.transform = 'translateY(0)';
            }
        })
        viewListen(blockThreeHeader, (isView) => {
            if (!isView) {
                glitch.style.color = 'rgb(2, 122, 191, 0.3)';
                isNumberBlock = false;
                fillElementWithRandomText(glitch);
                return
            }

            pageHeader.setColor('green')
            glitch.style.color = 'rgb(0, 255, 0, 0.2)';
            isNumberBlock = true;
            fillElementWithRandomText(glitch, '¥£¢€৳₮₢₦₭ƒ₱₠₲$₳₺')
        })
        viewListen(blockFourHeader, (isView) => {
            if (!isView) {
                pageHeader.setColor('green')
                return
            }
            pageHeader.setColor('white')
        })
    })

    onMount(() => {
        fillElementWithRandomText(glitch);

        onresize = (() => {
            fillElementWithRandomText(glitch);
        })

        onmousemove = ((event) => {
            let x = event.clientX / window.innerWidth;
            let y = event.clientY / window.innerHeight;
            glitch.style.transform = `scale(1.7) translate(-${x * 100}px, -${y * 100}px)`;
            if (isNumberBlock) {
                fillElementWithRandomText(glitch, '¥£¢€৳₮₢₦₭ƒ₱₠₲$₳₺')
            } else {
                fillElementWithRandomText(glitch);
            }
        })
    })

    function fillElementWithRandomText(element: HTMLElement, validCharacters: string = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_+-=[]{}|;:,.<>?') {
        let randomText = '';
        let charactersWidth = element.clientWidth / 10;
        let charactersHeight = element.clientHeight / 6;
        for (let i = 0; i < charactersWidth * charactersHeight; i++) {
            randomText += validCharacters.charAt(Math.floor(Math.random() * validCharacters.length));
        }

        element.innerText = randomText;
    }
</script>

<container>
    <Header bind:this={pageHeader}/>
    <div id="glitch" bind:this={glitch}/>
    <div class="block-wrapper">
        <block-one>
            <h1 class="top-header" bind:this={header}>EXPERIENCED</h1>
            <p class="top-description" bind:this={description}>LEVEL UP YOUR DISCORD EXPERIENCE</p>
            <a class="home-button" bind:this={inviteButton} href="https://discord.com/api/oauth2/authorize?client_id=1035970092284002384&permissions=0&scope=bot%20applications.commands">INVITE</a>
            <a class="home-button" bind:this={inviteButton} href="https://github.com/sponsors/randomairborne">DONATE</a>
            <a class="home-button" bind:this={inviteButton} href="terms">TERMS</a>
            <a class="home-button" bind:this={inviteButton} href="privacy">PRIVACY</a>

        </block-one>
    </div>

    <div class="block-wrapper">
        <block-two bind:this={gradient}>
            <h1 class="information-header" bind:this={blockTwoHeader}>Customizable Cards</h1>
            <p class="information-description" bind:this={blockTwoDescription}>Create your own cards with custom images
                and text</p>
            <img bind:this={img} src="https://cdn.loudbook.dev/files/card.png" alt="example card" width="700px"/>
        </block-two>
    </div>

    <div class="block-wrapper">
        <block-three>
            <h1 bind:this={blockThreeHeader} class="paywall-header">No NFTs or Paywalls</h1>
            <p class="paywall-description">Free. Forever. The goal of this project is not, was not, and will not be to
                generate revenue in any form.</p>
        </block-three>
    </div>

    <div class="block-wrapper">
        <img class="os-background" src="https://cdn.loudbook.dev/files/github.svg" alt="GitHub Logo" width="400px"/>
        <block-four>
            <h1 bind:this={blockFourHeader} class="os-header">Open Source</h1>
            <p class="os-description">This project is open source and available on <a href="github.com">GitHub</a>. Feel
                free to contribute or fork the project.</p>
        </block-four>
    </div>
</container>

<style lang="scss">
  #glitch {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background: #000;
    z-index: -1;
    color: rgb(2, 122, 191, 0.3);
    font-family: "JetBrains Mono", monospace;
    font-size: 15px;
    line-height: 15px;
    word-wrap: break-word;
    word-break: break-all;
    transform: scale(1.7);

    transition: color 0.5s cubic-bezier(.51, .54, 0, 1);
  }

  container {
    scroll-snap-type: y mandatory;
    overflow: scroll;
    height: 100vh;
  }

  .block-wrapper {
    position: relative;
    overflow: hidden;
    scroll-snap-align: start;
  }

  block-one {
    display: flex;
    flex-direction: column;
    justify-content: center;
    height: 100vh;
    width: 100vw;
    margin: 0;
    background-repeat: no-repeat;
    background-position: 100px 100px;

    a {
      transition: color 0.5s ease, transform 0.5s ease, opacity 0.5s ease;

      display: block;
      color: #00a1ff;
      font-family: 'Gabarito', sans-serif;
      font-weight: 900;
      font-size: 2vw;
      line-height: 2vw;
      margin-left: 0.6vw;
      width: 10vw;
      opacity: 0;
      transform: translateY(20px);

      &:active {
        transform: scale(0.96);
      }

      &:hover {
        opacity: 0.8;
      }

      @media (max-width: 700px) {
        font-size: 4vw;
        line-height: 4vw;
      }
    }
  }

  block-two {
    transition: background-size 1s cubic-bezier(0.45, 0, 0, 0.98), background-position 1s cubic-bezier(0.45, 0, 0, 0.98);

    display: flex;
    flex-direction: column;
    justify-content: center;
    align-content: center;
    width: 100vw;
    height: 100vh;
    background: rgb(0, 161, 255);
    background: linear-gradient(180deg, rgba(0, 161, 255, 0) 0%, rgba(0, 0, 0, 1) 47%);
    margin: 0;
    overflow: scroll;
  }

  block-three {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-content: center;
    width: 100vw;
    height: 100vh;

    background: rgb(0, 0, 0);
    background: linear-gradient(180deg, rgba(0, 0, 0, 1) 0%, rgba(0, 0, 0, 0) 50%, rgba(0, 0, 0, 1) 100%);
  }

  block-four {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-content: center;
    width: 100vw;
    height: 100vh;
    overflow: hidden;

    background-color: black;
  }

  .top-header {
    display: inline-block;
    color: #00a1ff;
    font-family: 'Gabarito', sans-serif;
    font-weight: 900;
    font-size: 13vw;
    margin-top: 0;
    margin-bottom: 0;
    line-height: 9vw;

    transition: opacity 1s cubic-bezier(.51, .54, 0, 1), transform 1s cubic-bezier(.51, .54, 0, 1);
  }

  .top-description, .information-description, .paywall-description, .os-description {
    display: block;
    color: #00a1ff;
    font-family: 'Gabarito', sans-serif;
    font-weight: 500;
    font-size: 2vw;
    line-height: 2vw;
    opacity: 0;
    margin: 0 0 0 0.5vw;

    animation: fadeUp 0.7s ease forwards;
    animation-delay: 0.2s;

    @media (max-width: 700px) {
      font-size: 4vw;
      line-height: 4vw;
    }
  }

  .information-header, .paywall-header, .os-header {
    display: block;
    color: #00a1ff;
    font-family: 'Gabarito', sans-serif;
    font-weight: 900;
    font-size: 4vw;
    margin-top: 0;
    margin-bottom: 0;
    line-height: 8vw;
    text-align: center;
    opacity: 1;

    transition: opacity 1s cubic-bezier(.51, .54, 0, 1), transform 1s cubic-bezier(.51, .54, 0, 1);

    @media (max-width: 700px) {
      font-size: 6vw;
      line-height: 12vw;
    }
  }

  .information-description {
    text-align: center;
    display: block;
    transition: opacity 1s cubic-bezier(.51, .54, 0, 1), transform 1s cubic-bezier(.51, .54, 0, 1);
    animation: none;
    opacity: 0.9;

    @media (max-width: 700px) {
      font-size: 3vw;
    }
  }

  block-two img {
    height: auto;
    width: 60%;
    justify-self: center;
    align-self: center;
    opacity: 0;

    transition: opacity 1s cubic-bezier(.51, .54, 0, 1), transform 1s cubic-bezier(.51, .54, 0, 1);
    transform: translateY(20px);

    @media (max-width: 700px) {
      width: 90%;
    }
  }

  .paywall-header {
    color: rgb(0, 255, 0, 1)
  }

  .paywall-description {
    color: rgb(0, 255, 0, 0.9);
    text-align: center;
  }

  .os-header {
    color: white;
  }

  .os-description {
    color: white;
    text-align: center;
    padding: 0 10px 10px;
  }

  .os-description a {
    display: inline-block;
    transition: color 0.5s ease, transform 0.5s ease;

    color: white;
    text-decoration: underline;

    &:hover {
      color: #e6e6e6;
    }

    &:active {
      transform: scale(0.96);
    }
  }

  .os-background {
    opacity: 0.2;
    position: absolute;
    left: 0;
    top: 0;
    width: 100%;
    height: 100%;
    transform: scale(1.5);
    overflow: hidden;

    animation: randomFlicker 1s infinite;
  }

  @keyframes fadeUp {
    from {
      opacity: 0;
      transform: translateY(20px);
    }
    to {
      opacity: 0.7;
      transform: translateY(0);
    }
  }

  @keyframes fadeUpFull {
    from {
      opacity: 0;
      transform: translateY(20px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
</style>