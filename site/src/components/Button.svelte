<script lang="ts">
	export let loading = false;
    export let type: "button" | "reset" | "submit" | null | undefined = 'button';
    export let disabled = false;
</script>

<button disabled={disabled || loading} on:click type={type} class="text-center text-sm font-medium focus:ring-4 focus:outline-none inline-flex items-center justify-center px-5 py-2.5 text-white bg-primary-700 hover:bg-primary-800 dark:bg-primary-600 dark:hover:bg-primary-700 focus:ring-primary-300 dark:focus:ring-primary-800 rounded w-fit self-center">
    <slot></slot>
    {#if loading}
        <div
            class="inline-block h-4 w-4 animate-spin rounded-full border-4 border-solid border-current border-r-transparent align-[-0.125em] motion-reduce:animate-[spin_1.5s_linear_infinite] ml-2"
            role="status"
        >
            <span
                class="!absolute !-m-px !h-px !w-px !overflow-hidden !whitespace-nowrap !border-0 !p-0 ![clip:rect(0,0,0,0)]"
                >...</span
            >
        </div>
    {/if}
</button>