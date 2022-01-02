<ul>
    <li>
        <div><strong>Middleware authorize</strong></div>
        <div>
            The status of jwt token and database is not same after logout.
            <ul>
                <li>expired_at in database is already updated</li>
                <li>expired_at in token isn't updated.</li>
            </ul>
            <div>need to mark the token invalid.</div>
        </div>
    </li>
</ul>