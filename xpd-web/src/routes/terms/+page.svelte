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
        <h2 id="section-i-binding-of-the-terms">Section I - Binding of the Terms</h2>
        <p>Experienced is owned and operated by valkyrie_pilot. These Terms set forth the terms and conditions under which you may
            use our Discord bot and services as offered by us. This bot offers users the ability to track their activity by a proxy
            of &quot;experience&quot;. By inviting this bot via Discord, you affirm that you have read, understood, and agree to be bound by
            these Terms.</p>
        <h2 id="section-ii-requirements-for-use">Section II - Requirements for Use</h2>
        <p>In order to receive our services, you must be at least 13 years of age, or of the legal age of majority in your
            jurisdiction, and possess the legal authority, right and freedom to enter into these Terms as a binding agreement. You
            are not allowed to use receive services if doing so is prohibited in your country or under any law or regulation
            applicable to you.</p>
        <h2 id="section-iii-retention-of-right-to-change-offering">Section III - Retention of Right to Change Offering</h2>
        <p>We may, without prior notice, change the services or stop providing services or any features of the services we offer.
            We may permanently or temporarily terminate or suspend access to the services without notice, refund, or liability for
            any reason, or for no reason.</p>
        <h2 id="section-iv-warranty-responsibility">Section IV - Warranty Responsibility</h2>
        <p>Because of the open source nature of the services we offer, and the likelihood of there being bugs, the services are
            provided on a &quot;as is&quot; basis and without warranties of any kind concerning the work, including without limitation
            merchantability, fitness for a particular purpose, absence of defects or errors, accuracy, non-infringement of
            intellectual property rights other than copyright as stated in section 3E of the Microsoft Public License</p>
        <h2 id="section-v-right-to-suspend-or-cancel-user-account">Section V - Right to Suspend or Cancel User Account</h2>
        <p>We may permanently or temporarily terminate or suspend your access without refund to the service without notice and
            liability for any reason, including if in our sole determination you violate any provision of these Terms or any
            applicable law or regulations. You may discontinue use and request to cancel your account at any time.</p>
        <h2 id="section-vi-indemnification">Section VI - Indemnification</h2>
        <p>You agree to indemnify and hold valkyrie_pilot harmless from any demands, loss, liability, claims, or expenses (
            including attorney&#39;s fees) made against them by any third party due to, or arising out of, or in connection with your
            use of the services offered.</p>
        <h2 id="section-vii-limitation-of-liability">Section VII - Limitation of Liability</h2>
        <p>To the maximum extent permitted by applicable law, in no event shall valkyrie_pilot, be liable for any indirect,
            punitive, incidental, special, consequential or exemplary damages, including without limitation, damages for loss of
            profits, goodwill, use, data or other intangible losses, arising out of or relating to the use of, or inability to use,
            the service.</p>
        <p>In addition, the maximum extent permitted by applicable law valkyrie_pilot assumes no liability or responsibility for
            any (i) errors, mistakes, or inaccuracies of content; (ii) personal injury or property damage, of any nature whatsoever,
            resulting from your access to or use of our service; and (iii) any unauthorized access to or use of our secure servers
            and/or any and all personal information stored therein.</p>
        <h2 id="section-xiii-purpose-of-use-of-our-services">Section XIII - Purpose of Use of our Services</h2>
        <p>You agree to refrain from using our Services to spread potentially harmful theories or general misinformation that could
            be destructive to public safety/health. In addition, you will not abuse our Services to engage in the promotion of
            cryptocurrencies, or to deny our Services to other users.</p>
        <h2 id="section-ix-right-to-change-and-modify-terms">Section IX - Right to Change and Modify Terms</h2>
        <p>We reserve the right to modify these terms from time to time at our sole discretion. Therefore, you should review this
            page periodically. When we change the Terms in a material manner, we will notify you that material changes have been
            made to the Terms. Your continued use of the Bot or our other services after any such change constitutes your acceptance
            of the new Terms. If you do not agree to any of these terms or any future version of the Terms, do not use or access (or
            continue to access) the service. In addition, cancel the storage of your data with the gdpr delete Discord command.</p>
        <h2 id="section-x-preference-of-law-and-dispute-resolution">Section X - Preference of Law and Dispute Resolution</h2>
        <p>These Terms, the rights and remedies provided hereunder, and any and all claims and disputes related hereto and/or to
            the services, shall be governed by, construed under and enforced in all respects solely and exclusively in accordance
            with the internal substantive laws of Colorado in the United States of America, without respect to its conflict of laws
            principles. Any and all such claims and disputes shall be brought in, and you hereby consent to them being decided
            exclusively by a court of competent jurisdiction located in Vail, Colorado, United States of America</p>
        <p>Last updated: April 20, 2024</p>
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