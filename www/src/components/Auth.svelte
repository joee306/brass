<script lang="ts">
    import Signup from "./Signup.svelte";

    let message: string = "";
    let username: string = "";
    let password: string = "";
    let signup = false;
    const login = async () => {
        if (username === "" || password === "") return;
        const { location } = window;
        const res = await fetch(
            `http://${location.host}/api/login/${username}/${password}`,
            {
                method: "GET",
            }
        );

        if (set_message(await res.text())) {
            username = "";
            password = "";
            window.location.replace("home");
        }
    };

    const set_message = (msg: string) => {
        if (msg != "Session set") {
            message = msg;
            return false;
        }
        return true;
    };
</script>

{#if signup}
    <Signup />
{:else}
    <div class="flex items-center justify-center h-screen max-w-fulll">
        <div class="grid">
            <input
                bind:value={username}
                class="m-2 p-1 rounded-lg outline-none dark:bg-gray-800 dark:text-slate-100"
                placeholder="Your Username or Email"
            />
            <input
                type="password"
                bind:value={password}
                class="m-2 p-1 rounded-lg outline-none dark:bg-gray-800 dark:text-slate-100"
                placeholder="Your Password"
            />
            <div class="grid grid-cols-2">
                <button
                    class="m-2 p-1 rounded-lg outline-none hover:text-blue-400 active:bg-neutral-950 dark:bg-gray-800 dark:text-slate-100"
                    on:click={login}
                >
                    Login
                </button>
                <button
                    class="m-2 p-1 rounded-lg outline-none hover:text-blue-400 active:bg-neutral-950 dark:bg-gray-800 dark:text-slate-100"
                    on:click={() => {
                        signup = true;
                    }}
                >
                    Signup
                </button>
            </div>
            <span
                class="m-2 p-1 rounded-lg outline-none text-center relative text-red-500"
            >
                {message}
            </span>
        </div>
    </div>
{/if}
