<script lang="ts">
    import Profile from "./Profile.svelte";

    let username: String;
    let password: String;
    let email: String;
    let message: string;
    const signup = async () => {
        if (username === "" || password === "") return;
        const { location } = window;
        const res = await fetch(
            `http://${location.host}/api/signup/${email}/${username}/${password}`,
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

<div>
    <div class="flex items-center justify-center h-screen max-w-fulll">
        <div class="grid grid-cols-2">
            <div class="grid">
                <span class="text-zinc-50 p-1">username : </span>
                <input
                    class="mb-2 p-1 rounded-lg outline-none dark:bg-gray-800 dark:text-slate-100"
                    type="text"
                    name="username"
                    bind:value={username}
                />
                <span class="text-zinc-50 p-1">password : </span>
                <input
                    class="mb-2 p-1 rounded-lg outline-none dark:bg-gray-800 dark:text-slate-100"
                    type="password"
                    name="password"
                    bind:value={password}
                />
                <span class="text-zinc-50 p-1">email : </span>
                <input
                    class="mb-2 p-1 rounded-lg outline-none dark:bg-gray-800 dark:text-slate-100"
                    type="email"
                    name="email"
                    bind:value={email}
                />
                <div class="grid grid-cols-2">
                    <button
                        class="mr-1 p-1 rounded-lg outline-none hover:text-blue-400 active:bg-neutral-950 dark:bg-gray-800 dark:text-slate-100"
                    >
                        Back
                    </button>
                    <button
                        class="ml-1 p-1 rounded-lg outline-none hover:text-blue-400 active:bg-neutral-950 dark:bg-gray-800 dark:text-slate-100"
                        on:click={signup}
                    >
                        Signup
                    </button>
                </div>
            </div>
            <Profile />
        </div>
    </div>
</div>
