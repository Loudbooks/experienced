<script lang="ts">
    import Header from "../../components/Header.svelte";
    import {onMount} from "svelte";

    let header: Header;

    onMount(() => {
        header.setColor('white');
    });
</script>

<main>
    <Header bind:this={header} alwaysShow={true}/>
    <content>
        <h2 id="what-data-is-collected-">What data is collected?</h2>
        <h3 id="experienced-only-stores-the-bare-minimum-required-to-operate-this-includes-the-following-">Experienced
            only
            stores the bare minimum required to operate. This includes the following:</h3>
        <ul>
            <li>User IDs</li>
            <li>Guild IDs</li>
            <li>Per-guild configuration</li>
            <li>Per-user configuration</li>
            <li>Approximate message counts for users and guilds</li>
        </ul>
        <h4 id="anonymous-aggregated-statistics-about-bot-function-including-but-not-limited-to-">Anonymous, aggregated
            statistics about bot function, including, but not limited to:</h4>
        <ul>
            <li>Discord Gateway events: only the total bot-wide count of events is stored.</li>
            <li>Guild count</li>
            <li>User count</li>
            <li>None of this aggregated data can be linked back to a specific guild.</li>
        </ul>
        <h2 id="for-what-purposes-is-this-data-stored-">For what purposes is this data stored?</h2>
        <p>This data is stored for various purposes, including:</p>
        <ul>
            <li>Operating the leveling aspect of this bot</li>
            <li>Storing configuration settings persistently</li>
            <li>Providing feedback on what areas need to be worked upon</li>
        </ul>
        <h2 id="how-is-this-data-used-">How is this data used?</h2>
        <ul>
            <li>Any personally identifiable data is only used to provide core functions of the bot.</li>
            <li>Anonymous, aggregated stats are used for various purposes, including, but not limited to, providing
                feedback
                on the
                growth of the bot.
            </li>
        </ul>
        <h2 id="who-is-the-data-shared-with-">Who is the data shared with?</h2>
        <ul>
            <li>Potentially personally identifiable data is available to with no one except the core developer
                (valkyrie_pilot), and
                they will only view this information if required.
            </li>
            <li>Anonymous, aggregated stats are shared with everyone who wishes to view them.</li>
        </ul>
        <h2 id="who-can-i-contact-with-concerns-">Who can I contact with concerns?</h2>
        <p>You can contact the developer via email at valk (at) randomairborne.dev</p>
        <h2 id="how-can-i-request-deletion-of-my-data-">How can I request deletion of my data?</h2>
        <p>You may contact the core developers via the mediums specified above and request we delete your data.
            The Discord bot also contains a command, <code>/gdpr delete</code>, which will delete all data associated
            with
            you.</p>
    </content>
</main>

<style lang="scss">
  main {
    margin: 0;
    color: white;
    font-family: Gabarito, sans-serif;
  }

  content {
    display: block;
    padding: 70px 2% 2%;
    font-size: 12px;
  }
</style>