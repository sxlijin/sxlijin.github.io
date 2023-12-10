function Writer (doc, opts)
  return pandoc.utils.stringify(doc.meta)
end
