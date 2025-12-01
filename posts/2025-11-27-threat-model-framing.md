Put this together when I was designing an encryption scheme for user data.

## Threat Modeling Manifesto ([link](https://www.threatmodelingmanifesto.org/))

- What are we working on?
- What can go wrong?
- What are we going to do about it?
- Did we do a good enough job?

## Five Functions ([NIST](https://www.nist.gov/cyberframework/getting-started/online-learning/five-functions))

- identify:
    - risk identification will rely on the threat modeling in this document
- protect:
    - via secure design and safeguards implemented in accordance with our threat model
    - our use of infisical means that we’re usually not storing keys in plaintext on our machines, and they’re in the macOS keyring
- detect:
    - KMS will give us audit logs around access to user secrets with timestamp and service vs human principal (but not the end-user authority!)
- respond
    - flush secrets? or tag them as exposed?
    - we need to rotate our own wrapper key
    - we need to re-encrypt all encrypted secrets at rest
- recover
    - we need to message to customers that they should strongly consider rotating their keys because we were compromised
## STRIDE

From Adam Shostack, who wrote the [canonical book](https://shostack.org/books/threat-modeling-book) on threat modeling:

|**Threat Category**|**Violates**|**Examples**|
|---|---|---|
|**S**poofing|Authenticity|An attacker steals the authentication token of a legitimate user and uses it to impersonate the user.|
|**T**ampering|Integrity|An attacker abuses the application to perform unintended updates to a database.|
|**R**epudiation|Non-repudiability|An attacker manipulates logs to cover their actions.|
|**I**nformation Disclosure|Confidentiality|An attacker extract data from a database containing user account info.|
|**D**enial of Service|Availability|An attacker locks a legitimate user out of their account by performing many failed authentication attempts.|
|**E**levation of Privileges|Authorization|An attacker tampers with a JWT to change their role.|

## Cyber Kill Chains

How attacks work, from [Building Secure and Reliable Systems](https://google.github.io/building-secure-and-reliable-systems/raw/ch02.html#cyber_kill_chains) by Adkins et al:

| _Reconnaissance_: Surveilling a target victim to understand their weak points.                  | Attacker uses a search engine to find the email addresses of employees at a target organization.                                                                                                          | Educate employees about online safety.                                                                                                   |
| ----------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| _Entry_: Gaining access to the network, systems, or accounts necessary to carry out the attack. | Attacker sends phishing emails to employees that lead to compromised account credentials. The attacker then signs in to the organization’s virtual private network (VPN) service using those credentials. | Use two-factor authentication (such as security keys) for the VPN service.Only permit VPN connections from organization-managed systems. |
| _Lateral movement_: Moving between systems or accounts to gain additional access.               | Attacker remotely logs in to other systems using the compromised credentials.                                                                                                                             | Permit employees to log in to only their own systems.Require two-factor authentication for login to multiuser systems.                   |
| _Persistence_: Ensuring ongoing access to compromised assets.                                   | Attacker installs a backdoor on the newly compromised systems that provides them with remote access.                                                                                                      | Use application allowlisting that permits only authorized software to run.                                                               |
| _Goals_: Taking action on attack goals.                                                         | Attacker steals documents from the network and uses the remote access backdoor to exfiltrate them.                                                                                                        | Enable least privileged access to sensitive data and monitoring of employee accounts.                                                    |