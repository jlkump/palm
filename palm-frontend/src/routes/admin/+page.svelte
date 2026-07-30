<script>
    import UserListItem from "$lib/components/UserListItem.svelte";
</script>

<style>
    .wrapper {
        border-radius: 8px;
        background-color: #fff;
        box-shadow: 0 2px 10px 0 rgba(0,0,0, 0.1);

        width: 80vw;
        padding: 24px;
    }

    h2 {
        padding: 0;
        margin: 0;
    }

    .user-list {
        display: flex;
        flex-direction: column;

        margin: 24px;
        gap: 16px;
    }
</style>

<div class="wrapper">
    <h2>Users</h2>
    <div>
        <input type="text" placeholder="Search..."/>
    </div>
    <div class="user-list">
        <UserListItem/>
        <UserListItem user_name="Landon Kump" user_email="LandonKump@collectivechimes.com"/>
        <UserListItem user_name="Orion Chapman" user_email="OrionChapman@collectivechimes.com"/>
        <UserListItem user_name="Bailey Bignall" user_email="BaileyBignall@collectivechimes.com"/>
    </div>
</div>