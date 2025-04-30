<script>
    import { page } from '$app/stores';

    $: userId = $page.params.id;</script>

<h1>Home</h1>
<h1>Bin # {userId}</h1>
