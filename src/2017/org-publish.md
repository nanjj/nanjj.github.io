# Publish Github Pages via Org Mode

> Created at <2017-10-24 Fri 12:30>
[Google]: http://www.google.com/search?q=%s
[Hugo]: https://gohugo.io/
[Jekyll]: https://jekyllrb.com/

Now switched from [Jekyll], [Hugo] to Emacs Org Publish. How to write
blog using org-publish.

## Customization

First we need to customize variable `org-publish-project-alist`:
  
```lisp
("blog"
 :base-directory "."
 :publishing-directory "."
 :base-extension "org"
 :recursive t
 :publishing-function org-html-publish-to-html
 :headline-levels 4
 :auto-preamble t
 :section-numbers nil
 :makeindex nil
 :html-head-include-scripts nil
 :html-head-include-default-style nil
 :auto-sitemap t
 :sitemap-filename "index.org"
 :sitemap-title "Thatched Cottage"
 :sitemap-sort-files anti-chronologically
 :sitemap-format-entry org-publish-sitemap-time-entry
 :html-postamble nil)
```

My `sitemap-format-entry`, add timestamp before the sitemap entry:

```
(defun org-publish-sitemap-time-entry (entry style project)
  (format "%s %s"
          (format-time-string
           "[%Y-%m-%d %a %H:%s]"
           (org-publish-find-date entry project))
          (org-publish-sitemap-default-entry entry style project)))
```

Set `org-html-head` to(`customize-variable org-html-head`) to use my

`org.css`:
```
<link rel="stylesheet" type="text/css" href="css/org.css" />
```

## The Project

```
.
├── .git
├── .nojekyll
├── README.md
├── css
│   ├── DroidSans.ttf
│   ├── DroidSansMono.ttf
│   ├── DroidSerif.ttf
│   ├── fonts.css
│   └── org.css
├── dummynet.html
├── dummynet.org
├── growfs.html
├── growfs.org
├── hdiutil.html
├── hdiutil.org
├── index.html
├── index.org
├── org-publish.html
└── org-publish.org
```
