<script lang="ts">
    import {onMount} from "svelte";

    export let content: string;
    export let title: string;

    let cell: HTMLElement;

    onMount(() => {
        let inView: boolean = false;
        const observer = new IntersectionObserver((entries) => {
            entries.forEach((entry) => {
                if (entry.isIntersecting) {
                    if (!inView) {
                        cell.style.opacity = '1';
                        cell.style.transform = 'translateY(0)';

                        inView = true;
                    }
                }
            });
        }, {
            threshold: 0.5
        });

        observer.observe(cell);
    })
</script>

<cell-container>
    <cell bind:this={cell}>
        <h1>{title}</h1>
        <p>{content}</p>
    </cell>
</cell-container>


<style lang="scss">
  cell-container {
    transition: transform 1s cubic-bezier(0.45,0,0,0.98);

    &:hover {
      transform: scale(1.01) translateY(-20px);

      @media (max-width: 768px) {
        transform: scale(1.01);
      }
    }
  }
  cell {
    transition: opacity 1s ease, transform 1s cubic-bezier(0.45,0,0,0.98);
    display: flex;
    flex-direction: column;
    padding: 10px;
    justify-content: space-between;

    border-radius: 10px;
    background-color: #141414;
    border: 1px solid #00a1ff;
    box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
    width: 300px;
    height: 275px;
    opacity: 0;
    transform: translateY(100px);

  }

  h1 {
    font-size: 30px;
    text-align: center;
    color: #00a1ff;
    font-family: Gabarito, sans-serif;
    font-weight: 900;
    padding: 10px;
  }

  p {
    font-size: 20px;
    color: #027abf;
    font-family: Gabarito, sans-serif;
    font-weight: 400;
    padding: 10px;
    padding-bottom: 0;
    justify-self: end;
    align-self: end;
  }
</style>