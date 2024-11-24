window.plugins.set(
  "reggub_helper",
  {
    init: () => {
      return [];
    },
    update: (msg, model) => {
      return [];
    },
    view: model => {
      const openTab = (tab) => {
        // Declare all variables
        var i, tabcontent, tablinks;

        // Get all elements with class="tabcontent" and hide them
        tabcontent = document.getElementsByClassName("tabcontent");
        for (i = 0; i < tabcontent.length; i++) {
          tabcontent[i].style.display = "none";
        }

        // Get all elements with class="tablinks" and remove the class "active"
        tablinks = document.getElementsByClassName("tablinks");
        for (i = 0; i < tablinks.length; i++) {
          tablinks[i].className = tablinks[i].className.replace(" active", "");
        }

        // Show the current tab, and add an "active" class to the button that opened the tab
        document.getElementById(tab).style.display = "block";
      };
      const val = model.find(([k, _]) => k === "reggub-tab-btn");
      if (val !== undefined) {
        const tab = val[1].Str;
        openTab(tab);
      }
      return {
        kind: "Frag",
        kids: [],
        text: null,
        attrs: [],
      }
    },
  }
);