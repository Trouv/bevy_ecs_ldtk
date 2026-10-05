// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><a href="index.html">Introduction</a></li><li class="chapter-item expanded affix "><li class="part-title">Tutorials</li><li class="chapter-item expanded "><a href="tutorials/tile-based-game/index.html"><strong aria-hidden="true">1.</strong> Tile-based Game</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="tutorials/tile-based-game/create-your-ldtk-project.html"><strong aria-hidden="true">1.1.</strong> Create your LDtk project</a></li><li class="chapter-item expanded "><a href="tutorials/tile-based-game/spawn-your-ldtk-project-in-bevy.html"><strong aria-hidden="true">1.2.</strong> Spawn your LDtk project in Bevy</a></li><li class="chapter-item expanded "><a href="tutorials/tile-based-game/add-gameplay-to-your-project.html"><strong aria-hidden="true">1.3.</strong> Add gameplay to your project</a></li></ol></li><li class="chapter-item expanded "><div><strong aria-hidden="true">2.</strong> Platformer</div></li><li class="chapter-item expanded affix "><li class="part-title">Explanation</li><li class="chapter-item expanded "><a href="explanation/level-selection.html"><strong aria-hidden="true">3.</strong> Level Selection</a></li><li class="chapter-item expanded "><a href="explanation/game-logic-integration.html"><strong aria-hidden="true">4.</strong> Game Logic Integration</a></li><li class="chapter-item expanded "><a href="explanation/anatomy-of-the-world.html"><strong aria-hidden="true">5.</strong> Anatomy of the World</a></li><li class="chapter-item expanded "><div><strong aria-hidden="true">6.</strong> Plugin Schedule</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">7.</strong> Asset Model</div></li><li class="chapter-item expanded "><a href="explanation/limitations.html"><strong aria-hidden="true">8.</strong> Limitations</a></li><li class="chapter-item expanded affix "><li class="part-title">How-To Guides</li><li class="chapter-item expanded "><div><strong aria-hidden="true">9.</strong> Register Bundles for Intgrid Tiles and LDtk Entities</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">10.</strong> Process Entities Further with Blueprints</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">11.</strong> Combine Tiles into Larger Entities</div></li><li class="chapter-item expanded "><a href="how-to-guides/create-bevy-relations-from-ldtk-entity-references.html"><strong aria-hidden="true">12.</strong> Create Bevy Relations from LDtk Entity References</a></li><li class="chapter-item expanded "><a href="how-to-guides/respawn-levels-and-worlds.html"><strong aria-hidden="true">13.</strong> Respawn Levels and Worlds</a></li><li class="chapter-item expanded "><a href="how-to-guides/make-level-selection-follow-player.html"><strong aria-hidden="true">14.</strong> Make LevelSelection Follow Player</a></li><li class="chapter-item expanded "><div><strong aria-hidden="true">15.</strong> Animate Tiles</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">16.</strong> Camera Logic</div></li><li><ol class="section"><li class="chapter-item expanded "><div><strong aria-hidden="true">16.1.</strong> Implement Fit-Inside Camera</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">16.2.</strong> Implement Fit-Around Camera</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">16.3.</strong> Implement Parallax</div></li></ol></li><li class="chapter-item expanded "><div><strong aria-hidden="true">17.</strong> Retrieve Field Instance Data</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">18.</strong> Retrieve Loaded Level Data</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">19.</strong> Compile to WASM</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">20.</strong> Compile Headless</div></li><li class="chapter-item expanded "><a href="how-to-guides/migration-guides/index.html"><strong aria-hidden="true">21.</strong> Migration Guides</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="how-to-guides/migration-guides/migrate-from-0.8-to-0.9.html"><strong aria-hidden="true">21.1.</strong> Migrate from 0.8 to 0.9</a></li><li class="chapter-item expanded "><a href="how-to-guides/migration-guides/migrate-from-0.9-to-0.10.html"><strong aria-hidden="true">21.2.</strong> Migrate from 0.9 to 0.10</a></li><li class="chapter-item expanded "><a href="how-to-guides/migration-guides/migrate-from-0.10-to-0.11.html"><strong aria-hidden="true">21.3.</strong> Migrate from 0.10 to 0.11</a></li></ol></li><li class="chapter-item expanded "><li class="spacer"></li><li class="chapter-item expanded affix "><a href="api-reference.html">API Reference</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0].split("?")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
