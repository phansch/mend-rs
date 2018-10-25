# frozen_string_literal: true

require 'bundler/inline'

gemfile do
  source 'https://rubygems.org'
  gem 'faktory_worker_ruby'
end

faktory = Faktory::Client.new(url: 'tcp://localhost:7419')
faktory.push(jid: SecureRandom.hex(12), queue: 'default', jobtype: 'pull_request', args: [{hello: :world, from: :ruby}])

