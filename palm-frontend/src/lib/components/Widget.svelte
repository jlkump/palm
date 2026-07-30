<script lang="ts">
  let { children, style = "" } = $props();
</script>

<style>
    .widget {
        border-radius: 8px;
        background-color: #fff;
        box-shadow: 0 2px 10px 0 rgba(0,0,0, 0.1);
    
        display: block;
        margin: 24px;
        padding: 48px;
    }

    .widget > * + * {
        margin-top: 1.25em;
    }

    .widget h2 {
        font-size: 1.5rem;
        font-weight: 700;
        line-height: 1.25;
    }

</style>

<div class="widget" style={style}>
    {@render children()}
</div>