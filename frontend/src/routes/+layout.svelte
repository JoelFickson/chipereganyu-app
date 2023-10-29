<script>
    import '../app.css';
    import NavBar from "$lib/components/NavBar.svelte";
</script>

<NavBar/>

<div class='min-h-screen flex justify-center bg-gradient-to-b from-primary-color-500 to-accent-color-50'>
    <slot/>
</div>

