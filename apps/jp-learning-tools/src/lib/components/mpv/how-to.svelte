<div class="space-y-8">
  <section>
    <h2 class="text-2xl font-bold mb-4">Make a simple sentence card</h2>

    <p class="mb-4">
      To make a card from the currently visible subtitle line, press
      <kbd
        class="px-2 py-1 text-xs font-semibold text-gray-800 bg-gray-100 border border-gray-200 rounded dark:bg-gray-600 dark:text-gray-100 dark:border-gray-500"
        >Ctrl+n</kbd
      >.
    </p>

    <p class="mb-2">To make a card from two or more subtitle lines:</p>

    <ol class="list-decimal list-inside space-y-2 ml-2 mb-4">
      <li>
        Press
        <kbd
          class="px-2 py-1 text-xs font-semibold text-gray-800 bg-gray-100 border border-gray-200 rounded dark:bg-gray-600 dark:text-gray-100 dark:border-gray-500"
          >a</kbd
        >
        to open advanced menu.
      </li>
      <li>
        Press
        <kbd
          class="px-2 py-1 text-xs font-semibold text-gray-800 bg-gray-100 border border-gray-200 rounded dark:bg-gray-600 dark:text-gray-100 dark:border-gray-500"
          >c</kbd
        >
        to start interactive selection.
      </li>
      <li>
        Seek to the previous/next subtitle with
        <kbd
          class="px-2 py-1 text-xs font-semibold text-gray-800 bg-gray-100 border border-gray-200 rounded dark:bg-gray-600 dark:text-gray-100 dark:border-gray-500"
          >Shift+h</kbd
        >
        and
        <kbd
          class="px-2 py-1 text-xs font-semibold text-gray-800 bg-gray-100 border border-gray-200 rounded dark:bg-gray-600 dark:text-gray-100 dark:border-gray-500"
          >Shift+l</kbd
        >.
      </li>
      <li>
        Press
        <kbd
          class="px-2 py-1 text-xs font-semibold text-gray-800 bg-gray-100 border border-gray-200 rounded dark:bg-gray-600 dark:text-gray-100 dark:border-gray-500"
          >n</kbd
        >
        to create a new card.
      </li>
    </ol>

    <p class="mb-2">
      After the card is created, you can find it by typing <code
        class="px-1.5 py-0.5 text-sm font-mono bg-gray-100 rounded dark:bg-gray-800"
        >added:1</code
      > in the Anki Browser. The card doesn't contain dictionary definitions. You
      need to add them yourself afterward, using software like GoldenDict, Qolibri,
      etc.
    </p>
  </section>

  <section>
    <h2 class="text-2xl font-bold mb-4">Quick Card Creation</h2>

    <p class="mb-4">
      The goal of the quick card creation menu is to streamline a specific flow
      for media with <strong>well-timed subtitles</strong>:
    </p>

    <ol class="list-decimal list-inside space-y-2 ml-2 mb-4">
      <li>
        Add a new note to Anki via
        <a
          href="https://tatsumoto.neocities.org/blog/setting-up-goldendict.html"
          class="text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300 underline"
          >GoldenDict</a
        >,
        <a
          href="https://tatsumoto.neocities.org/blog/setting-up-yomichan.html"
          class="text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300 underline"
          >Rikaitan</a
        >
        or any other tool that works with AnkiConnect.
      </li>
      <li>Update the note via mpvacious.</li>
    </ol>

    <p class="mb-4">
      To update the most recently added card from the currently visible subtitle
      line, press
      <kbd
        class="px-2 py-1 text-xs font-semibold text-gray-800 bg-gray-100 border border-gray-200 rounded dark:bg-gray-600 dark:text-gray-100 dark:border-gray-500"
        >gg</kbd
      >.
    </p>

    <p class="mb-2">To make a card from two or more subtitle lines:</p>

    <ol class="list-decimal list-inside space-y-2 ml-2 mb-4">
      <li>
        Press
        <kbd
          class="px-2 py-1 text-xs font-semibold text-gray-800 bg-gray-100 border border-gray-200 rounded dark:bg-gray-600 dark:text-gray-100 dark:border-gray-500"
          >g</kbd
        >
        to open the quick card creation menu.
      </li>
      <li>
        Press any number
        <kbd
          class="px-2 py-1 text-xs font-semibold text-gray-800 bg-gray-100 border border-gray-200 rounded dark:bg-gray-600 dark:text-gray-100 dark:border-gray-500"
          >[2-9]</kbd
        >. This number corresponds to the number of lines to create the card
        from.
      </li>
    </ol>

    <p class="mb-4">
      Note:
      <kbd
        class="px-2 py-1 text-xs font-semibold text-gray-800 bg-gray-100 border border-gray-200 rounded dark:bg-gray-600 dark:text-gray-100 dark:border-gray-500"
        >g1</kbd
      >
      is also valid. However,
      <kbd
        class="px-2 py-1 text-xs font-semibold text-gray-800 bg-gray-100 border border-gray-200 rounded dark:bg-gray-600 dark:text-gray-100 dark:border-gray-500"
        >gg</kbd
      >
      is an additional bind to further streamline the most common scenario.
    </p>

    <p class="mb-4">
      For example,
      <kbd
        class="px-2 py-1 text-xs font-semibold text-gray-800 bg-gray-100 border border-gray-200 rounded dark:bg-gray-600 dark:text-gray-100 dark:border-gray-500"
        >g2</kbd
      >
      creates a card using 2 subtitle lines.
    </p>

    <p class="mb-2">
      Like the advanced menu, you can also update multiple cards:
    </p>

    <ol class="list-decimal list-inside space-y-2 ml-2 mb-4">
      <li>
        Press
        <kbd
          class="px-2 py-1 text-xs font-semibold text-gray-800 bg-gray-100 border border-gray-200 rounded dark:bg-gray-600 dark:text-gray-100 dark:border-gray-500"
          >Alt+g</kbd
        >
        to select the number of cards for quick card creation.
      </li>
      <li>
        Press any number
        <kbd
          class="px-2 py-1 text-xs font-semibold text-gray-800 bg-gray-100 border border-gray-200 rounded dark:bg-gray-600 dark:text-gray-100 dark:border-gray-500"
          >[2-9]</kbd
        >. This number corresponds to the number of cards to update.
      </li>
      <li>
        Press any number
        <kbd
          class="px-2 py-1 text-xs font-semibold text-gray-800 bg-gray-100 border border-gray-200 rounded dark:bg-gray-600 dark:text-gray-100 dark:border-gray-500"
          >[1-9]</kbd
        >
        again. This is the number of lines.
      </li>
    </ol>

    <p class="mb-4">
      Note: upon completing the note update, the selected number of cards resets
      back to the default of one.
    </p>

    <p class="mb-4">
      For example,
      <kbd
        class="px-2 py-1 text-xs font-semibold text-gray-800 bg-gray-100 border border-gray-200 rounded dark:bg-gray-600 dark:text-gray-100 dark:border-gray-500"
        >(Alt+g)22</kbd
      >
      would update the last 2 notes using 2 subtitle lines.
    </p>
  </section>

  <section>
    <h2 class="text-2xl font-bold mb-4">
      Modifying cards added with GoldenDict
    </h2>

    <p class="mb-4">
      You can add a card first using GoldenDict, and then append an audio clip
      and a picture to it.
    </p>

    <p class="mb-4">
      <strong>Note:</strong> the only version of GoldenDict that can create Anki
      cards with configurable fields is
      <a
        href="https://github.com/xiaoyifang/goldendict-ng"
        class="text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300 underline"
        >xiaoyifang's goldendict</a
      >. Read
      <a
        href="https://tatsumoto-ren.github.io/blog/setting-up-goldendict.html"
        class="text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300 underline"
        >Setting up GoldenDict</a
      >
      and
      <a
        href="https://github.com/xiaoyifang/goldendict-ng/blob/staged/website/docs/topic_anki.md"
        class="text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300 underline"
        >How to connect with Anki</a
      >
      if you are new to GoldenDict.
    </p>

    <p class="mb-2">
      To send subtitles from <code
        class="px-1.5 py-0.5 text-sm font-mono bg-gray-100 rounded dark:bg-gray-800"
        >mpv</code
      >
      directly to GoldenDict, append the following line to
      <code
        class="px-1.5 py-0.5 text-sm font-mono bg-gray-100 rounded dark:bg-gray-800"
        >subs2srs.conf</code
      >:
    </p>

    <pre
      class="bg-gray-100 dark:bg-gray-800 p-4 rounded mb-4 overflow-x-auto"><code
        class="text-sm font-mono">autoclip_method=goldendict</code
      ></pre>

    <p class="mb-4">
      <strong>Note:</strong> If
      <code
        class="px-1.5 py-0.5 text-sm font-mono bg-gray-100 rounded dark:bg-gray-800"
        >goldendict</code
      >
      is not in the PATH, you have to
      <a
        href="https://wiki.archlinux.org/title/Environment_variables#Per_user"
        class="text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300 underline"
        >add it to the PATH</a
      >.
    </p>

    <ol class="list-decimal list-inside space-y-2 ml-2 mb-4">
      <li>
        Press
        <kbd
          class="px-2 py-1 text-xs font-semibold text-gray-800 bg-gray-100 border border-gray-200 rounded dark:bg-gray-600 dark:text-gray-100 dark:border-gray-500"
          >a</kbd
        >
        to open
        <code
          class="px-1.5 py-0.5 text-sm font-mono bg-gray-100 rounded dark:bg-gray-800"
          >advanced menu</code
        >.
      </li>
      <li>
        Press
        <kbd
          class="px-2 py-1 text-xs font-semibold text-gray-800 bg-gray-100 border border-gray-200 rounded dark:bg-gray-600 dark:text-gray-100 dark:border-gray-500"
          >t</kbd
        >
        to toggle the
        <code
          class="px-1.5 py-0.5 text-sm font-mono bg-gray-100 rounded dark:bg-gray-800"
          >autoclip</code
        > option.
      </li>
    </ol>

    <p class="mb-4">
      Now as subtitles appear on the screen, they will be immediately sent to
      GoldenDict instead of the system clipboard.
    </p>

    <ol class="list-decimal list-inside space-y-2 ml-2 mb-4">
      <li>Open GoldenDict.</li>
      <li>
        Play a video in <code
          class="px-1.5 py-0.5 text-sm font-mono bg-gray-100 rounded dark:bg-gray-800"
          >mpv</code
        >.
      </li>
      <li>
        When you find an unknown word, select the definition text, right-click
        and select "send word to anki" to make a card, or press
        <kbd
          class="px-2 py-1 text-xs font-semibold text-gray-800 bg-gray-100 border border-gray-200 rounded dark:bg-gray-600 dark:text-gray-100 dark:border-gray-500"
          >Ctrl</kbd
        >+<kbd
          class="px-2 py-1 text-xs font-semibold text-gray-800 bg-gray-100 border border-gray-200 rounded dark:bg-gray-600 dark:text-gray-100 dark:border-gray-500"
          >Shift</kbd
        >+<kbd
          class="px-2 py-1 text-xs font-semibold text-gray-800 bg-gray-100 border border-gray-200 rounded dark:bg-gray-600 dark:text-gray-100 dark:border-gray-500"
          >N</kbd
        >.
      </li>
      <li>
        If you have the
        <a
          href="../README.md#new-note-timer"
          class="text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300 underline"
          >new note timer</a
        >
        enabled, mpvacious will automatically add media to your card.
        <br /><br />
        Otherwise, go back to mpv and add an image and an audio clip to the card
        you've just made by pressing
        <kbd
          class="px-2 py-1 text-xs font-semibold text-gray-800 bg-gray-100 border border-gray-200 rounded dark:bg-gray-600 dark:text-gray-100 dark:border-gray-500"
          >m</kbd
        >
        while the
        <code
          class="px-1.5 py-0.5 text-sm font-mono bg-gray-100 rounded dark:bg-gray-800"
          >advanced menu</code
        >
        is open. Pressing
        <kbd
          class="px-2 py-1 text-xs font-semibold text-gray-800 bg-gray-100 border border-gray-200 rounded dark:bg-gray-600 dark:text-gray-100 dark:border-gray-500"
          >Shift+m</kbd
        >
        will overwrite any existing data in media fields.
      </li>
    </ol>

    <div class="my-6">
      <a
        href="https://github.com/Ajatt-Tools/mpvacious/assets/69171671/0fc02d24-d320-4d2c-b7a9-cb478e9f0067"
        class="text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300 underline break-all"
      >
        https://github.com/Ajatt-Tools/mpvacious/assets/69171671/0fc02d24-d320-4d2c-b7a9-cb478e9f0067
      </a>
    </div>

    <p class="mb-4">
      Don't forget to set the right timings and join lines together if the
      sentence is split between multiple subs. To do it, enter interactive
      selection by pressing
      <kbd
        class="px-2 py-1 text-xs font-semibold text-gray-800 bg-gray-100 border border-gray-200 rounded dark:bg-gray-600 dark:text-gray-100 dark:border-gray-500"
        >c</kbd
      >
      and seek to the next or previous subtitle.
    </p>

    <p>
      To pair Mecab and GoldenDict, install
      <a
        href="https://github.com/Ajatt-Tools/gd-tools"
        class="text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300 underline"
        >gd-tools</a
      >.
    </p>
  </section>
</div>
