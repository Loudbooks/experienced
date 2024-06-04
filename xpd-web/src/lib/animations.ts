export function revealWord(element: HTMLElement, original: string, delay: number, inter: number = 3, takes: number = 20) {
    const text = original;
    let count = -takes + 1;
    let index = 0;

    setTimeout(() => {
        const interval = setInterval(() => {
            let newRandomString = '';

            for (let i = 0; i < text.substring(index).length; i++) {
                newRandomString += randomCharacter();
            }

            if (index <= text.length) {
                element.innerText = text.substring(0, index) + newRandomString

                if (count % takes === 0) {
                    index++;
                }

                count++;
            } else {
                clearInterval(interval);
            }
        }, inter);
    }, delay);
}

function randomCharacter(): string {
    const characters = 'QWERTYUIOPASDFGHJKLZXCVBNM@#$%^&*';
    return characters.charAt(Math.floor(Math.random() * characters.length));
}

export function viewListen(element: HTMLElement, callback: (inView: boolean) => void) {
    let inView = false;

    const observer = new IntersectionObserver((entries) => {
        entries.forEach((entry) => {
            if (entry.isIntersecting) {
                if (!inView) {
                    callback(true);

                    inView = true;
                }
            } else {
                callback(false)

                inView = false;
            }
        });
    }, {
        threshold: 0.5
    });

    observer.observe(element);
}