; From the PHP extension for Zed, https://github.com/zed-extensions/php
; (Apache-2.0, see LICENSE-APACHE in this folder).

((text) @injection.content
  (#set! injection.language "html")
  (#set! injection.combined))

((comment) @injection.content
  (#set! injection.language "comment"))

((heredoc_body)
  (heredoc_end) @injection.language) @injection.content

((nowdoc_body)
  (heredoc_end) @injection.language) @injection.content
