<script lang="ts">
    import default_profile from "$lib/assets/DefaultProfile.png"
    import ProfileImage from "./ProfileImage.svelte";
    let { user_name = "John Doe", user_email = "JohnDoe@collectivechimes.com", user_profile = default_profile } = $props();
</script>

<style>
    .user-item {
        display: flex;
        flex-direction: row;
        justify-content: space-between;

        cursor: pointer;

        padding: 8px;
        border-radius: 12px;
    }

    .user-item:hover {
        background-color: var(--item);
    }

    .user-item > * {
        display: flex;
        align-items: center;
    }

    .user-info {
        width: 40vw;
    }

    .profile {
        width: 24px;
        height: 24px;
        border-radius: 50%;
        background: gray;
        margin-right: 1em;
    }

    .email {
        font-size: 1em;
        color: var(--email);
    }

    .list-container {
        display: flex;
        justify-content: space-between;
    }

    .service-list, .menu-list {
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: space-between;

        font-size: 1.5em;
    }

    .service-list {
        margin-right: 4em;
    }

    .service-list > *, .menu-list > * {
        margin: 0 0.5em 0 0.5em;
        padding: 8px;
        border-radius: 8px;
    }

    .service-list > *:hover, .menu-list > *:hover {
        color: black;
        background-color: var(--paper);
    }

    .menu-list > * {
        border: 2px solid var(--item-border);
    }

    .menu-list > *:hover {
        border: 2px solid var(--item);
    }

    .disabled {
        color: var(--disabled);
    }

    .warning {
        color: var(--warning);
    }

    .error {
        color: var(--error);
        background-color: var(--error-bg);
    }

    @media (max-width: 931px) {
        .user-item {
            flex-direction: column;
        }
    }

    @media (max-width: 500px) {
        .list-container {
            flex-direction: column;
        }

        .service-list {
            margin-right: 0;
        }
    }

    p {
        margin: 0;
        text-overflow: ellipsis;
        overflow: clip;
    }
</style>

<div class="user-item">
    <div class="user-info">
        <ProfileImage size="32" style="margin-right: 1em" image_src={user_profile} />
        <div style="max-width: 50vw;">
            <p>{user_name}</p>
            <p class="email"><em>{user_email}</em></p>
        </div>
    </div>
    <div class="list-container">
        <div class="service-list">
            <i class="fa-brands fa-trello disabled"></i>
            <i class="fa-brands fa-forgejo error"></i>
            <i class="fa-solid fa-book"></i>
        </div>
        <div class="menu-list">
            <i class="fa-solid fa-pen-to-square"></i>
            <i class="fa-solid fa-box-archive"></i>
        </div>
    </div>
</div>