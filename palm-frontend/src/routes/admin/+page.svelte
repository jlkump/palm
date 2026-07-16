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

    .menu-bar {
        display: grid;
        grid-template-columns: auto auto auto auto;

        margin: 16px;
    }

    .user-list {
        display: flex;
        flex-direction: column;

        margin: 24px;
        gap: 16px;
    }
    
    .user-item {
        display: grid;
        grid-template-columns: auto auto auto auto;
    }

    .user-item > * {
        display: flex;
        align-items: center;
    }

    .profile {
        width: 24px;
        height: 24px;
        border-radius: 50%;
        background: gray;
        margin-right: 1em;
    }

    button {
        height: 32px;
        width: 52px;

        border: 0;
        border-radius: 12px;
        cursor: pointer;
        background: var(--accent);
        color: #fff;
        font-size: 15px;
        font-weight: 600;
        transition: filter .18s,transform .12s;
    }

    button:focus-visible {
        outline: 3px solid var(--accent-button);
        outline-offset: 3px;
    }
</style>

<div class="wrapper">
    <h2>Users</h2>
    <div class="menu-bar">
        <h3>Name</h3>
        <h3>Email</h3>
        <h3>Services</h3>
        <div></div>
    </div>
    <div class="user-list">
        <div class="user-item">
            <div>
                <div class="profile"></div>
                User's name
            </div>
            <div>User's email</div>
            <div>User's services</div>
            <div>
                <button>Edit</button>
            </div>
        </div>
    </div>
</div>