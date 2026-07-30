<script lang="ts">
    import default_profile from "$lib/assets/DefaultProfile.png"
    let { image_src = default_profile, size = "64", style = "" } = $props();
</script>

<style>
    .profile {
        border-radius: 50%;
        border: 3px solid var(--accent);
        padding: 2px;
    }
</style>

<img class="profile" src="{image_src}" {style} width="{size}px" height="{size}px"/>