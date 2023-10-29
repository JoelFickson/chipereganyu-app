<script>

    import Routes from "$lib/routes.ts";
    import GoGrabber from 'svelte-icons/go/GoGrabber.svelte'


    let navbarOpen = false;
    const routes = Routes

    function toggleNavbar() {
        navbarOpen = !navbarOpen;
    }
</script>

<nav class="relative flex flex-wrap items-center justify-between py-2 bg-primary-color-500 text-white ">
    <div class="container flex flex-wrap items-center justify-between px-4 mx-auto">
        <div class="relative flex justify-between w-full lg:w-auto lg:static lg:block lg:justify-start">
            <a class="flex   items-center justify-center py-2 mr-4 font-bold leading-relaxed uppercase"
               href="/">
                Chipereganyu
            </a>
            <button
                    class="block px-3 py-1 text-xl leading-none bg-transparent border border-transparent border-solid rounded outline-none cursor-pointer lg:hidden focus:outline-none"
                    on:click={toggleNavbar}
            >
                <div class="icon">
                    <GoGrabber/>
                </div>
            </button>
        </div>
        <div class={`lg:flex flex-grow items-center ${navbarOpen ? 'flex' : 'hidden'}`}>
            <div class="flex flex-col items-center justify-between gap-4 lg:flex-row lg:ml-auto">
                {#each routes as myRoute (myRoute.path)}
                    <a
                            href={myRoute.path}
                            class="flex  items-center  justify-center py-2 mr-4 font-bold leading-relaxed"
                    >
                        {myRoute.name}
                    </a>
                {/each}

            </div>
        </div>
    </div>
</nav>
