## Skills Manager Site Redesign

### Direction

Rebuild the current marketing site into a more refined editorial landing page inspired by `tenor.finance`, but adapted for a desktop developer tool rather than a fintech product.

### Visual System

- Warm off-white canvas with subtle cream gradients
- Oversized serif hero headline with restrained sans-serif body copy
- Minimal top navigation with quiet spacing and a rounded primary CTA
- Abstract floating "skill objects" instead of finance coins
- A ribbed, architectural ground plane to anchor the hero
- Reduced card density and less dashboard-like UI chrome

### Product Translation

The reference site sells composure, trust, and polish. For Skills Manager, that same language should communicate:

- order across scattered skill files
- confidence in local-first workflows
- premium desktop-tool focus instead of startup landing-page noise

### Layout

1. Minimal header with logo, three nav links, GitHub, and primary release CTA.
2. Hero split:
   - left: large serif statement, product paragraph, primary and secondary CTAs
   - right: sculptural visual made of floating skill sheets, disks, and a simplified app object
3. Bottom hero foundation:
   - layered ribbed terrain similar in spirit to the reference image
4. Follow-up sections:
   - features in editorial rows instead of repetitive card grids
   - supported agents as a cleaner strip
   - download section kept practical but visually integrated

### Content Changes

- Replace the current generic "One place to manage..." hero with sharper copy
- Emphasize local-first organization and direct file editing
- Keep release downloads prominent
- Keep the macOS unsigned note, but style it as product guidance instead of a warning block

### Responsive Behavior

- Mobile keeps the serif hero but shortens line length and collapses navigation
- Hero visual stacks below copy and becomes a tighter composition
- Buttons remain prominent in the first viewport

### Implementation Notes

- Rework `site/index.html` structure instead of incremental tweaks
- Replace most of `site/styles.css`
- Keep `site/script.js` minimal for anchor scrolling only
- Preserve current deployment flow and URL structure
