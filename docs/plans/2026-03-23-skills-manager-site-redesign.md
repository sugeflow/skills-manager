# Skills Manager Site Redesign Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Rebuild the Skills Manager static site into a more polished editorial landing page inspired by Tenor's visual language while preserving download-first messaging and the existing deployment flow.

**Architecture:** Keep the site as a pure static page under `site/`, but replace the current dashboard-like hero and dense card layout with a typography-first editorial composition, floating product objects, and a lighter information hierarchy. Preserve the existing deployment pipeline so the redesign is only a markup and CSS replacement, not a tooling change.

**Tech Stack:** Static HTML, CSS, minimal JavaScript, Python preview server, rsync deployment

---

### Task 1: Replace the page structure

**Files:**
- Modify: `site/index.html`

**Step 1: Rewrite the header and hero markup**

- Introduce a simpler header with logo, nav, GitHub link, and a rounded CTA.
- Replace the current faux-app three-column hero with a new split layout containing:
  - large serif headline
  - product paragraph
  - CTA row
  - floating sculptural visual elements
  - ribbed ground plane

**Step 2: Rework the feature and download sections**

- Replace repetitive cards with more editorial blocks and a cleaner download matrix.
- Keep supported agents and FAQ, but reduce visual clutter.

**Step 3: Keep existing release links intact**

- Preserve all current GitHub release asset URLs.

**Step 4: Commit**

```bash
git add site/index.html
git commit -m "refactor: redesign marketing site structure"
```

### Task 2: Rebuild the visual system

**Files:**
- Modify: `site/styles.css`

**Step 1: Replace the current CSS with a new editorial system**

- Define a lighter neutral palette
- Rebuild typography scale
- Rebuild hero layout
- Add floating visual object styles
- Add ribbed terrain background styles

**Step 2: Improve responsive behavior**

- Tighten mobile spacing
- Collapse the hero into a vertical flow
- Keep the first CTA visible early on smaller screens

**Step 3: Preserve reduced-motion behavior**

- Keep motion limited and gracefully disabled under `prefers-reduced-motion`

**Step 4: Commit**

```bash
git add site/styles.css
git commit -m "feat: add editorial visual system for marketing site"
```

### Task 3: Verify and deploy

**Files:**
- Modify: `package.json`
- Reuse: `scripts/deploy-site.sh`

**Step 1: Build locally**

Run: `npm run build`
Expected: Vite build completes successfully

**Step 2: Preview the static site**

Run: `npm run site:preview`
Expected: site loads on `http://localhost:4173`

**Step 3: Deploy upload bundle**

Run: `./scripts/deploy-site.sh`
Expected: updated site assets sync to `zhichu:~/skill-upload`

**Step 4: Promote to live directory**

Run:

```bash
ssh zhichu 'printf "21\n" | sudo -S rsync -av --delete ~/skill-upload/ /var/www/skills-manager-site/'
```

Expected: updated files sync to `/var/www/skills-manager-site`

**Step 5: Verify production**

Run: `curl -s https://skill.sugeflow.com | sed -n '1,20p'`
Expected: returned HTML contains `Skills Manager`

**Step 6: Commit**

```bash
git add package.json site scripts
git commit -m "feat: ship redesigned marketing site"
```
