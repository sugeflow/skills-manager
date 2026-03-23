const links = document.querySelectorAll('a[href^="#"]');

for (const link of links) {
  link.addEventListener("click", (event) => {
    const target = link.getAttribute("href");
    if (!target || target === "#") return;
    const node = document.querySelector(target);
    if (!node) return;
    event.preventDefault();
    node.scrollIntoView({ behavior: "smooth", block: "start" });
  });
}
