<script>
    import '../app.css';
    import NavBar from "$lib/components/NavBar.svelte";
</script>

<NavBar/>

<div class='min-h-screen flex justify-center bg-gradient-to-b from-emerald-50 to-stone-400'>
    <slot/>
</div>

