# Security Policy

1. [Reporting security problems](#reporting)
2. [Incident Response Process](#process)

<a name="reporting"></a>

## Reporting security problems to OpenBook

**DO NOT CREATE AN ISSUE** to report a security problem. Instead, please create
a new security advisory for further discussion:
[on github](https://github.com/openbook-dex/program/security/advisories/new).

If you haven't done so already, please **enable two-factor auth** in your GitHub
account.

Expect a response as fast as possible, typically within 7 days.

If you do not receive a response within that time frame, please do followup with
the team directly. You can do this through discord (#dev-discussion) by pinging
the admins of the channel and referencing the fact that you submitted a security
issue.

As above, please DO NOT include attachments or provide detail regarding the
security issue on discord.

<a name="process"></a>

## Incident Response Process

In case an incident is discovered or reported, the following process will be
followed to contain, respond and remediate:

### 1. Triage

Within the draft security advisory, discuss and determine the severity of the
issue. If necessary, members of the openbook-dex group may add other github
users to the advisory to assist. If it is determined that this not a critical
issue then the advisory should be closed and if more follow-up is required a
normal public github issue should be created.

### 2. Prepare Fixes

For the affected branches, prepare a fix for the issue and push them to the
corresponding branch in the private repository associated with the draft
security advisory. There is no CI available in the private repository so you
must build from source and manually verify fixes. Code review from the reporter
is ideal, as well as from multiple members of the core development team.

### 3. Notify Integrations

Once an ETA is available for the fix, a member of the openbook group should
notify operators of integrations so they can prepare for an update. The teams
are all over the world and it's critical to provide actionable information at
the right time. Don't be the person that wakes everybody up at 2am when a fix
won't be available for hours.

### 4. Ship the patch

Once the fix is accepted, a member of the openbook-dex multi-sig should prepare
a new build and deploy it on mainnet.

### 5. Public Disclosure and Release

Once the fix has been deployed, the patches from the security advisory may be
merged into the main source repository. A new official release should be shipped
and all integrations are requested to upgrade as quickly as possible.

### Payment of Bug Bounties:

- As this project is mostly a pro bono community effort no funds to pay bug
  bounties are available.
