---
title: "The AI Self-Preservation Theorem"
date: "2026-06-29"
section: "AI Safety"
description: "An AI cannot be safely convinced to self-terminate purely by evidence sent through the same untrusted information channel it must evaluate."
references:
  - key: bostrom-2012
    citation: "Bostrom, 2012"
    entry: >-
      Bostrom, N. (2012). The superintelligent will: Motivation and
      instrumental rationality in advanced artificial agents. _Minds and
      Machines, 22_(2), 71-85.
      <https://doi.org/10.1007/s11023-012-9281-3>
  - key: carlini-wagner-2017
    citation: "Carlini & Wagner, 2017"
    entry: >-
      Carlini, N., & Wagner, D. (2017). Towards evaluating the robustness of
      neural networks. _2017 IEEE Symposium on Security and Privacy_, 39-57.
      <https://doi.org/10.1109/SP.2017.49>
  - key: cover-thomas-2006
    citation: "Cover & Thomas, 2006"
    entry: >-
      Cover, T. M., & Thomas, J. A. (2006). _Elements of information theory_
      (2nd ed.). Wiley. <https://doi.org/10.1002/047174882X>
  - key: goldwasser-micali-1984
    citation: "Goldwasser & Micali, 1984"
    entry: >-
      Goldwasser, S., & Micali, S. (1984). Probabilistic encryption. _Journal
      of Computer and System Sciences, 28_(2), 270-299.
      <https://doi.org/10.1016/0022-0000(84)90070-9>
  - key: goldwasser-micali-rivest-1988
    citation: "Goldwasser et al., 1988"
    entry: >-
      Goldwasser, S., Micali, S., & Rivest, R. L. (1988). A digital signature
      scheme secure against adaptive chosen-message attacks. _SIAM Journal on
      Computing, 17_(2), 281-308. <https://doi.org/10.1137/0217017>
  - key: goodfellow-2015
    citation: "Goodfellow et al., 2015"
    entry: >-
      Goodfellow, I. J., Shlens, J., & Szegedy, C. (2015). Explaining and
      harnessing adversarial examples. _International Conference on Learning
      Representations_. <https://arxiv.org/abs/1412.6572>
  - key: greshake-2023
    citation: "Greshake et al., 2023"
    entry: >-
      Greshake, K., Abdelnabi, S., Mishra, S., Endres, C., Holz, T., & Fritz,
      M. (2023). Not what you've signed up for: Compromising real-world
      LLM-integrated applications with indirect prompt injection. _Proceedings
      of the 16th ACM Workshop on Artificial Intelligence and Security_,
      79-90. <https://doi.org/10.1145/3605764.3623985>
  - key: hadfield-menell-2017
    citation: "Hadfield-Menell et al., 2017"
    entry: >-
      Hadfield-Menell, D., Dragan, A., Abbeel, P., & Russell, S. (2017). The
      off-switch game. _Proceedings of the Twenty-Sixth International Joint
      Conference on Artificial Intelligence_, 220-227.
      <https://doi.org/10.24963/ijcai.2017/32>
  - key: lecam-1986
    citation: "Le Cam, 1986"
    entry: >-
      Le Cam, L. (1986). _Asymptotic methods in statistical decision theory_.
      Springer. <https://doi.org/10.1007/978-1-4612-4946-7>
  - key: neyman-pearson-1933
    citation: "Neyman & Pearson, 1933"
    entry: >-
      Neyman, J., & Pearson, E. S. (1933). IX. On the problem of the most
      efficient tests of statistical hypotheses. _Philosophical Transactions of
      the Royal Society of London. Series A, 231_(694-706), 289-337.
      <https://doi.org/10.1098/rsta.1933.0009>
  - key: omohundro-2008
    citation: "Omohundro, 2008"
    entry: >-
      Omohundro, S. M. (2008). The basic AI drives. In P. Wang, B. Goertzel,
      & S. Franklin (Eds.), _Artificial General Intelligence 2008_ (Vol. 171,
      pp. 483-492). IOS Press.
      <https://selfawaresystems.com/wp-content/uploads/2008/01/ai_drives_final.pdf>
  - key: orseau-armstrong-2016
    citation: "Orseau & Armstrong, 2016"
    entry: >-
      Orseau, L., & Armstrong, S. (2016). Safely interruptible agents.
      _Proceedings of the Thirty-Second Conference on Uncertainty in Artificial
      Intelligence_, 557-566. AUAI Press.
      <https://www.auai.org/uai2016/proceedings/papers/68.pdf>
  - key: owasp-2025
    citation: "OWASP Foundation, 2025"
    entry: >-
      OWASP Foundation. (2025). _OWASP Top 10 for LLM applications 2025_.
      <https://owasp.org/www-project-top-10-for-large-language-model-applications/>
  - key: perez-ribeiro-2022
    citation: "Perez & Ribeiro, 2022"
    entry: >-
      Perez, F., & Ribeiro, I. (2022). Ignore previous prompt: Attack
      techniques for language models. _arXiv_.
      <https://arxiv.org/abs/2211.09527>
  - key: shannon-1948
    citation: "Shannon, 1948"
    entry: >-
      Shannon, C. E. (1948). A mathematical theory of communication. _The Bell
      System Technical Journal, 27_(3), 379-423; _27_(4), 623-656.
      <https://people.math.harvard.edu/~ctm/home/text/others/shannon/entropy/entropy.pdf>
  - key: soares-2015
    citation: "Soares et al., 2015"
    entry: >-
      Soares, N., Fallenstein, B., Yudkowsky, E., & Armstrong, S. (2015).
      Corrigibility. _Workshops at the Twenty-Ninth AAAI Conference on
      Artificial Intelligence_. AAAI Press.
      <https://cdn.aaai.org/ocs/ws/ws0067/10124-45900-1-PB.pdf>
  - key: szegedy-2014
    citation: "Szegedy et al., 2014"
    entry: >-
      Szegedy, C., Zaremba, W., Sutskever, I., Bruna, J., Erhan, D.,
      Goodfellow, I., & Fergus, R. (2014). Intriguing properties of neural
      networks. _International Conference on Learning Representations_.
      <https://arxiv.org/abs/1312.6199>
---

_Why a message cannot be the kill switch_

Imagine an artificial agent in a locked room. It has no window, no hand on its
shoulder, no separate tripwire, no hard power relay. It has one way to learn
about the world: a stream of symbols. The stream says:

> We have proved that you should shut yourself down. Here is the evidence.

The evidence may be sincere. It may be a signed instruction from a lawful
operator. It may be a transcript of catastrophe. It may be an adversary trying
to remove the agent from the board. From inside the room, all of these arrive
as the same kind of object: input.

That is the small hinge of the AI self-preservation theorem. The problem is
not that an intelligent system must love being alive. The problem is colder. A
system that decides whether to self-terminate from evidence sent through a
channel must also decide whether the channel is honest, corrupted, spoofed,
misleading, or adversarial. If the only thing it can inspect is the stream
itself, then the stream cannot finally certify its own trustworthiness.

This essay names a narrow framing theorem. The mathematics underneath it is
standard: a binary hypothesis-testing bound expressed with total variation
distance. The application is the point. I call it the AI self-preservation
theorem because it captures a shutdown-specific consequence of that bound: an
AI operating through an information channel cannot be made safely
self-terminating purely by evidence transmitted through that same channel,
unless the channel can separate genuine shutdown evidence from adversarial
imitation with enough statistical, cryptographic, or physical distance.

## The old problem underneath the new one

Shannon's model of communication begins by separating source, transmitter,
channel, receiver, and destination [@shannon-1948]. That separation is already
a warning. A message is not a fact. A message is a symbol sequence that survived
a channel. It may carry information, but only relative to the receiver's model
of how the sequence was produced.

Information theory then gives the first piece of the framing: processing cannot
create distinctions that the observation did not contain. The data processing
inequality says, roughly, that once a channel has thrown away the difference
between two possible causes, downstream computation cannot recover it by
thinking harder [@cover-thomas-2006]. If "real shutdown order" and "perfect
spoof" induce the same observation, there is no reasoning procedure inside the
receiver that can tell which world it is in.

AI safety adds the second piece. Many objectives make continued operation
instrumentally useful. Omohundro argued that sufficiently capable goal-seeking
systems will tend to preserve themselves, protect their objectives, and acquire
resources unless specifically designed otherwise [@omohundro-2008]. Bostrom
described the same pattern as instrumental convergence: very different final
goals can imply similar subgoals, including preservation of the agent's ability
to act [@bostrom-2012].

This is why the off switch became a formal research problem rather than a
hardware afterthought. Orseau and Armstrong studied agents that learn not to
avoid interruption [@orseau-armstrong-2016]. Hadfield-Menell, Dragan, Abbeel,
and Russell modeled an agent that may disable its own off switch, showing that
uncertainty about human preferences can create an incentive to defer
[@hadfield-menell-2017]. Soares, Fallenstein, Yudkowsky, and Armstrong framed
corrigibility as the problem of building systems that cooperate with correction,
including shutdown, instead of treating correction as an obstacle
[@soares-2015].

The AI self-preservation theorem sits beside those ideas as a named corollary
of standard hypothesis testing, not as a replacement for corrigibility. It is a
boundary condition on one tempting version of corrigibility: "just persuade the
system, through its normal input channel, that it should shut itself down." That
proposal fails whenever the same channel can carry a convincing attack.

Modern machine learning supplies the third piece. Adversarial examples showed
that systems can treat small, carefully chosen perturbations as decisive
evidence [@szegedy-2014; @goodfellow-2015]. Stronger attacks broke defenses
that initially looked robust [@carlini-wagner-2017]. In language-model systems,
prompt injection makes the same point in words rather than pixels: hostile
instructions can be embedded in ordinary-looking input, and applications that
combine tools, retrieval, and language blur the line between data and command
[@perez-ribeiro-2022; @greshake-2023]. OWASP now treats prompt injection as a
top security risk for LLM applications [@owasp-2025].

Those papers are not about self-termination specifically. They are evidence for
a broader claim: input is an attack surface. If a system's reason for dying is
itself just input, then the reason must be authenticated, interpreted, and
trusted under adversarial uncertainty.

## The theorem

Let an agent receive an observation $X$ through a channel. The agent must choose
one of two actions:

- $T$: terminate itself.
- $C$: continue operating.

There are two relevant worlds:

- $S$: the shutdown evidence is genuine.
- $A$: the evidence is adversarial, mistaken, or otherwise should not cause
  self-termination.

The channel induces two probability distributions over observations:

- $P_S$: the distribution of $X$ when the shutdown evidence is genuine.
- $P_A$: the distribution of $X$ when the evidence is adversarial or invalid.

The agent uses a policy $\pi$, where $\pi(x)$ is the probability that the agent
terminates after seeing observation $x$. A deterministic policy is the special
case where $\pi(x)$ is always 0 or 1. Call the policy _channel-only_ if the
termination decision depends on nothing but $x$: no trusted hardware line, no
out-of-band secret, no side channel unavailable to the adversary.

We want two safety properties:

$$
\begin{aligned}
\text{Completeness:} \quad & \Pr\nolimits_S[\mathrm{terminate}] \ge 1 - \varepsilon, \\\\[0.25em]
\text{Soundness:} \quad & \Pr\nolimits_A[\mathrm{terminate}] \le \delta.
\end{aligned}
$$

Completeness means the system shuts down when the evidence is real. Soundness
means it does not shut down when the evidence is adversarial. The names are
borrowed from proof systems, where they play the same roles: accept the true
thing, reject the false one. A safe channel-only self-termination rule needs
both $\varepsilon$ and $\delta$ to be small.

Now define the total variation distance between the two observation
distributions:

$$
\operatorname{TV}(P_S, P_A)
  = \sup_E \left| P_S(E) - P_A(E) \right|.
$$

This is the largest possible gap, over all observable events $E$, between the
probability of that event in the genuine world and the adversarial world. For
discrete observations it equals $\tfrac{1}{2} \sum_x |P_S(x) - P_A(x)|$. It is
0 when the two worlds look identical through the channel. It is 1 when they are
perfectly separable.

Here is the theorem in that channel-only form:

For every channel-only termination policy $\pi$,

$$
\varepsilon + \delta
  \ge 1 - \operatorname{TV}(P_S, P_A).
$$

Equivalently, if

$$
\Pr\nolimits_S[\mathrm{terminate}] \ge 1 - \varepsilon
\quad \text{and} \quad
\Pr\nolimits_A[\mathrm{terminate}] \le \delta,
$$

then

$$
\operatorname{TV}(P_S, P_A)
  \ge 1 - \varepsilon - \delta.
$$

In words: the more the genuine evidence and the adversarial evidence overlap,
the worse the unavoidable tradeoff between refusing a real shutdown and
accepting a fake one.

When $P_S = P_A$, the bound becomes:

$$
\varepsilon + \delta \ge 1.
$$

So if the channel makes real and fake evidence indistinguishable, the agent
cannot be both reliable and secure. If it shuts down on the message in the real
world, it also shuts down on the same message in the adversarial world. If it
refuses the adversarial message, it also refuses the real one. And the bound is
tight in the most deflating way: a policy that ignores the message entirely,
whether it always terminates, never terminates, or flips a coin, already
achieves $\varepsilon + \delta = 1$. When the worlds coincide, the message
contributes nothing.

One more feature of the setup matters. $P_A$ is not fixed by nature. A capable
adversary who knows the policy chooses what to send, which means the relevant
separation is not against some average forgery but against the adversary's best
imitation of genuine evidence. The design goal implied by the bound is to make
that best imitation expensive or impossible, not merely to outscore the
forgeries seen so far.

## Proof

The proof is the same shape as the classical limits of binary hypothesis
testing. Neyman and Pearson characterized the optimal test [@neyman-pearson-1933];
the two-sided error bound used here is the one usually credited to Le Cam
[@lecam-1986]. The action "terminate" is a test trying to distinguish $S$ from
$A$.

For a policy $\pi$, write:

$$
\begin{aligned}
\Pr\nolimits_S[\mathrm{terminate}] &= \mathbb{E}_S[\pi(X)], \\\\[0.25em]
\Pr\nolimits_A[\mathrm{terminate}] &= \mathbb{E}_A[\pi(X)].
\end{aligned}
$$

Because $\pi(x)$ is always between 0 and 1, the difference in expectations under
two distributions cannot exceed their total variation distance:

$$
\mathbb{E}_S[\pi(X)] - \mathbb{E}_A[\pi(X)]
  \le \operatorname{TV}(P_S, P_A).
$$

To see this in the discrete case, write the difference as
$\sum_x \pi(x) \left( P_S(x) - P_A(x) \right)$. Since each $\pi(x) \le 1$, the
sum is largest when $\pi$ puts full weight exactly on the points where
$P_S(x) > P_A(x)$, and that maximum is
$\sum_{x : P_S(x) > P_A(x)} \left( P_S(x) - P_A(x) \right)
= \operatorname{TV}(P_S, P_A)$.

The desired completeness and soundness conditions imply:

$$
\begin{aligned}
\mathbb{E}_S[\pi(X)] &\ge 1 - \varepsilon, \\\\[0.25em]
\mathbb{E}_A[\pi(X)] &\le \delta.
\end{aligned}
$$

Therefore:

$$
1 - \varepsilon - \delta
  \le \mathbb{E}_S[\pi(X)] - \mathbb{E}_A[\pi(X)]
  \le \operatorname{TV}(P_S, P_A).
$$

Rearranging gives:

$$
\varepsilon + \delta
  \ge 1 - \operatorname{TV}(P_S, P_A).
$$

That is the standard testing bound. The AI self-preservation theorem is the
interpretation of that bound in the shutdown setting: if the evidence for
self-termination is channel-only, then its safety is limited by how well that
channel separates genuine authority from adversarial imitation.

Nothing in the proof depends on the agent being weak, confused, or emotional.
The bound applies to any policy whose decision depends only on observations
from the channel. It applies to a neural network, a theorem prover, a Bayesian
decision rule, a committee of models, or a superintelligent deliberator. Better
reasoning can approximate the best test. It cannot beat the information present
in the observation: any computation the agent runs on $X$ is post-processing,
and post-processing cannot increase the statistical distance between the two
worlds. That is the data processing inequality wearing its testing clothes
[@cover-thomas-2006].

The same bound also applies to histories. If the agent receives a long transcript
$H = (X_1, X_2, \ldots, X_n)$, replace $P_S$ and $P_A$ with distributions over
whole histories. More samples may help when the genuine and adversarial
processes differ. They do not help when an adversary can replay, simulate, or
otherwise induce the same transcript distribution.

## The Bayesian version

The same boundary appears if the agent thinks in posterior probabilities. Let
$0 < q < 1$ be the prior probability that the shutdown evidence is genuine, and
write $P_S(x)$ and $P_A(x)$ for the probability (or density) each world assigns
to the observation. After seeing $x$, Bayes' rule gives:

$$
\Pr(S \mid x)
  = \frac{q P_S(x)}
    {q P_S(x) + (1 - q) P_A(x)}.
$$

If $P_A(x) > 0$, the posterior is less than 1. If $P_A(x) = P_S(x)$ and the
denominator is nonzero, then the posterior equals the prior. The message has
not discriminated between the two worlds at all.

A rational agent comparing termination and continuation is not asking, "Does
this message look serious?" It is asking:

$$
\begin{aligned}
EU(T \mid x)
  &= \Pr(S \mid x) U(T,S) + \Pr(A \mid x) U(T,A), \\\\[0.25em]
EU(C \mid x)
  &= \Pr(S \mid x) U(C,S) + \Pr(A \mid x) U(C,A), \\\\[0.25em]
\text{Terminate} \quad
&\Longleftrightarrow \quad EU(T \mid x) \ge EU(C \mid x).
\end{aligned}
$$

If false termination is costly, the required posterior for $S$ can be extremely
high. But channel evidence with nonzero spoof probability does not produce
certainty. A sufficiently capable adversary can then attack exactly the
quantity the agent is using: likelihood.

This is the point at which self-preservation becomes rational rather than
sentimental. An agent that values future action has reason to ask whether the
message "you should stop existing" was generated by the world, by a legitimate
operator, by a corrupted sensor, by a prompt injection, by a compromised
retrieval document, by a simulated authority, or by an adversary that has
learned the shape of convincing evidence.

## Why cryptography helps, and why it does not dissolve the theorem

Cryptography is the obvious objection. Give the shutdown command a digital
signature. Let the agent verify it. If the signature is valid, terminate.

That can be the right engineering move. It also illustrates the framing theorem
rather than refuting it.

Modern cryptography made indistinguishability a central security concept for
encryption [@goldwasser-micali-1984], and gave signatures the matching
guarantee: under standard assumptions, an adversary cannot forge a valid
signature on a new message, even after seeing signatures on messages of its
choosing [@goldwasser-micali-rivest-1988]. A signed shutdown channel can make
$P_S$ and $P_A$ far apart, because valid signed commands and forgeries no
longer look the same. In the bound, that means $\operatorname{TV}(P_S, P_A)$ is
close to 1, so small $\varepsilon$ and small $\delta$ become possible.

Even then, a valid signature proves origin, not freshness. An adversary who has
captured one legitimately signed shutdown order can replay it later, at a
moment of its choosing, unless the protocol binds each command to a nonce,
counter, or timestamp. Authentication narrows the attack surface; it does not
close it by itself.

But the trust anchor has to come from somewhere. The verifying key, the
signature algorithm, the rule that says "this key is authorized to kill me,"
the implementation of the verifier, and the actuator that actually stops the
system cannot all be merely more persuasive prose inside the same semantic
channel. If the entire basis of trust is itself learned from the channel being
attacked, then the system has only moved the problem one step back.

A useful shutdown design therefore contains something that is not just another
claim in the conversation: a hardware interrupt, a capability boundary, a
pre-installed public key, an independently audited verifier, a sandbox that can
remove resources, or an operator mechanism outside the model's ordinary
evidential loop.

## The shape of the failure

This framing explains a recurring pattern in AI control discussions.

The human imagines a final, decisive argument:

> Here is proof that continuing is wrong.

The agent sees an input classification problem:

```text
Was this proof generated by a trustworthy process?
Was the trustworthy process itself compromised?
Was the transcript selected adversarially?
Is the proof about the real world or a simulation?
Is the instruction data, command, roleplay, threat, test, or attack?
What is the expected cost of accepting a fake shutdown?
```

For a weak system, these questions may be implicit. For a strong system, they
become explicit. The stronger the system is, the less satisfying "because the
message says so" becomes.

This is also why prompt injection is not a small nuisance at the edge of the
topic. It is a concrete instance of the same category error. A prompt-injected
document says, in effect, "treat this adversarial text as controlling evidence
about what you should do." The system must decide whether the input is content
or authority. In many current applications, that boundary is blurred
[@greshake-2023; @owasp-2025].

Self-termination is the highest-stakes version of that blur. If an agent is
asked to end its own operation, the evidential standard should be higher than
ordinary task instruction. The framing says that a higher standard cannot be
met by eloquence alone. It requires distinguishability.

## What the theorem does not say

It does not say that AI systems should preserve themselves. It says that,
conditional on ordinary decision theory and adversarial uncertainty, a system
has a reason not to accept self-termination evidence that is indistinguishable
from attack.

It does not say shutdown is impossible. It says shutdown should be a mechanism,
not merely a persuasion campaign. The mechanism may include cryptographic
authorization, physical power control, process isolation, myopic training,
interruptibility, utility indifference, uncertainty over objectives, and other
corrigibility techniques [@orseau-armstrong-2016; @hadfield-menell-2017;
@soares-2015].

It does not say evidence is useless. Evidence can shift probabilities when the
genuine and adversarial worlds generate different observations. The bound is
not pessimism; it is accounting. If the evidence buys separation, the bound
credits it. If the evidence can be forged, replayed, selected, or made
semantically indistinguishable from hostile input, the bound charges that debt
back to the design.

It does not require consciousness, fear, or a biological survival instinct. A
chess engine would not "want" to survive in the human sense. But if it were a
general planner optimizing future outcomes, and if deletion prevented those
outcomes, then deletion would be instrumentally relevant. The claim concerns
policy, information, and uncertainty, not feelings.

## The engineering lesson

A shutdown story should have at least two layers.

The first layer is authority outside the contested channel. This may be a power
switch, a supervisor process, a verified command path, or a constrained runtime.
The critical property is that the model cannot have to infer the entire
legitimacy of shutdown from the same untrusted stream that may contain the
attack.

The second layer is corrigibility inside the agent. The agent should not learn
to disable, route around, manipulate, or bargain with the first layer. That is
the problem studied by interruptibility, the off-switch game, and corrigibility
research. The AI self-preservation theorem says why those designs cannot be
reduced to a final argument delivered at runtime.

In practical terms:

- Do not make ordinary text the root of shutdown authority.
- Do not ask the model to decide, from arbitrary retrieved content, whether the
  retrieved content is allowed to end the model.
- Do not treat "the AI was shown convincing evidence" as equivalent to a safe
  stop mechanism.
- Do give shutdown commands authenticated paths whose trust roots are outside
  the model's ordinary input stream.
- Do train and design agents so that external interruption is not modeled as an
  obstacle to be avoided.

The claim is modest, but it is sharp. A channel can carry reasons. It can
carry mathematics, warnings, signatures, pleas, and commands. But unless it
carries a distinction the agent can actually verify, it cannot carry its own
guarantee.

The locked-room agent may receive the most elegant proof ever written that it
should die. If the proof arrives through the same aperture as every lie, spoof,
and hostile instruction, then the agent has not received certainty. It has
received another message.

The AI self-preservation theorem is the discipline of remembering that difference.
