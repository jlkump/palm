<script lang="ts">
    import ProfileImage from "$lib/components/ProfileImage.svelte";
    import Widget from "$lib/components/Widget.svelte";
</script>

<style>
    .profile-header {
        position: relative;
        display: flex; 
        justify-content: space-between;

        gap: 1em;
    }

    h1 {
        margin-bottom: 0;
    }
    h2 {
        margin-top: 0;
        font-size: 1em;
    }

    .copy {
        user-select: none;
        cursor: pointer;
        color: var(--email);
    }

    .copy:hover {
        color: var(--accent);
    }

    .edit {
        display: flex;
        font-size: 1.5em;
        position: absolute;
        top: 0;
        right: 0;
    }

    .edit > * {
        cursor: pointer;

        padding: 8px;
        border-radius: 8px;
    }

    .edit > *:hover {
        background-color: var(--item);
    }

    .service-list {
        margin-top: 1.5em;

        display: flex;
        flex-direction: row;
        flex-wrap: wrap;
        justify-content: space-around;

        gap: 1.5em;
    }

    .service {
        padding: 8px;

        font-size: 1.5em;
        user-select: none;
        cursor: pointer;

        border-radius: 16px;
    }

    .service:hover {
        background-color: var(--item);
    }

    .link {
        font-size: 0.8em;
    }


    @media (max-width: 931px) {
        .profile-header {
            flex-direction: column;
            align-items: center;
        }
    }

    @media (max-width: 550px) {
        h1 {
            font-size: 1.5em;
        }

        h2 {
            font-size: 0.8em;
        }

        .edit {
            flex-direction: column;
        }
    }

</style>

<Widget style="max-width: 50vw;">
    <div class="profile-header">
        <ProfileImage size="128"/>
        <div>
            <h1>John Doe</h1>
            <h2 class="copy"><em>JohnDoe@collectivechimes.com</em><sup><i class="fa-regular fa-copy"></i></sup></h2>
        </div>
        <div class="edit">
            <i class="fa-solid fa-pen-to-square"></i>
            <i class="fa-solid fa-user-gear"></i>
        </div>
    </div>
    <div class="service-list">
        <div class="service">
            <i class="fa-brands fa-trello"></i>
            Taiga
            <i class="link fa-solid fa-up-right-from-square"></i>
        </div>
        <div class="service">
            <i class="fa-brands fa-forgejo"></i>
            Forgejo
            <i class="link fa-solid fa-up-right-from-square"></i>
        </div>
        <div class="service">
            <i class="fa-solid fa-book"></i>
            Wiki
            <i class="link fa-solid fa-up-right-from-square"></i>
        </div>
    </div>
</Widget>