<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Hero Section -->
    <div class="relative bg-gradient-to-br from-custom-chocolat via-amber-700 to-orange-700 overflow-hidden">
      <div class="absolute inset-0 opacity-10">
        <div class="absolute inset-0" style="background-image: url('data:image/svg+xml,%3Csvg width=&quot;60&quot; height=&quot;60&quot; viewBox=&quot;0 0 60 60&quot; xmlns=&quot;http://www.w3.org/2000/svg&quot;%3E%3Cg fill=&quot;none&quot; fill-rule=&quot;evenodd&quot;%3E%3Cg fill=&quot;%23ffffff&quot; fill-opacity=&quot;0.4&quot;%3E%3Cpath d=&quot;M36 34v-4h-2v4h-4v2h4v4h2v-4h4v-2h-4zm0-30V0h-2v4h-4v2h4v4h2V6h4V4h-4zM6 34v-4H4v4H0v2h4v4h2v-4h4v-2H6zM6 4V0H4v4H0v2h4v4h2V6h4V4H6z&quot;/%3E%3C/g%3E%3C/g%3E%3C/svg%3E');" />
      </div>

      <div class="relative max-w-7xl mx-auto px-4 pt-24 pb-16 md:pt-32 md:pb-20">
        <div class="text-center text-white">
          <div class="inline-flex items-center gap-2 bg-white/20 backdrop-blur-xs px-4 py-2 rounded-full text-sm font-medium mb-6">
            <font-awesome-icon :icon="['fas', 'envelope']" class="w-4 h-4" />
            Parlons ensemble
          </div>
          <h1 class="text-3xl md:text-5xl font-bold mb-4 font-display">
            Contactez-nous
          </h1>
          <p class="text-lg md:text-xl text-white/90 max-w-2xl mx-auto">
            Une question, une suggestion ou une proposition de collaboration ? N'hésitez pas à nous écrire.
          </p>
        </div>
      </div>

      <div class="absolute bottom-0 left-0 right-0">
        <svg class="w-full h-12 md:h-16" viewBox="0 0 1440 54" fill="none" xmlns="http://www.w3.org/2000/svg" preserveAspectRatio="none">
          <path d="M0 22L60 16.7C120 11 240 1 360 0.7C480 1 600 11 720 16.7C840 22 960 22 1080 16.7C1200 11 1320 1 1380 0.7L1440 0V54H1380C1320 54 1200 54 1080 54C960 54 840 54 720 54C600 54 480 54 360 54C240 54 120 54 60 54H0V22Z" fill="#f9fafb"/>
        </svg>
      </div>
    </div>

    <!-- Breadcrumb -->
    <div class="bg-gray-50">
      <div class="max-w-7xl mx-auto px-4 py-4">
        <CommonBreadcrumbNav />
      </div>
    </div>

    <!-- Contenu -->
    <section class="max-w-7xl mx-auto px-4 md:px-8 py-12">
      <div class="grid grid-cols-1 lg:grid-cols-5 gap-8">
        <!-- Formulaire -->
        <div class="lg:col-span-3">
          <div class="bg-white rounded-2xl shadow-sm border border-gray-100 p-6 md:p-8">
            <h2 class="text-xl font-semibold text-gray-800 mb-6 font-display">Envoyez-nous un message</h2>

            <form @submit.prevent class="space-y-5">
              <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                <div>
                  <label for="nom" class="block text-sm font-medium text-gray-700 mb-1">Nom complet</label>
                  <input
                    id="nom"
                    type="text"
                    class="w-full border-2 rounded-lg p-2.5 border-gray-200 focus:border-custom-green focus:outline-hidden transition-colors"
                    placeholder="Votre nom"
                  />
                </div>
                <div>
                  <label for="email" class="block text-sm font-medium text-gray-700 mb-1">Email</label>
                  <input
                    id="email"
                    type="email"
                    class="w-full border-2 rounded-lg p-2.5 border-gray-200 focus:border-custom-green focus:outline-hidden transition-colors"
                    placeholder="votre@email.com"
                  />
                </div>
              </div>

              <div>
                <label for="sujet" class="block text-sm font-medium text-gray-700 mb-1">Sujet</label>
                <input
                  id="sujet"
                  type="text"
                  class="w-full border-2 rounded-lg p-2.5 border-gray-200 focus:border-custom-green focus:outline-hidden transition-colors"
                  placeholder="Sujet de votre message"
                />
              </div>

              <div>
                <label for="message" class="block text-sm font-medium text-gray-700 mb-1">Message</label>
                <textarea
                  id="message"
                  rows="5"
                  class="w-full border-2 rounded-lg p-2.5 border-gray-200 focus:border-custom-green focus:outline-hidden transition-colors resize-y"
                  placeholder="Décrivez votre demande..."
                ></textarea>
              </div>

              <button
                type="submit"
                class="w-full sm:w-auto px-8 py-3 bg-custom-green text-white rounded-lg hover:bg-custom-green/90 transition-colors font-medium flex items-center justify-center gap-2"
              >
                <font-awesome-icon :icon="['fas', 'paper-plane']" />
                Envoyer le message
              </button>
            </form>
          </div>
        </div>

        <!-- Informations de contact -->
        <div class="lg:col-span-2 space-y-6">
          <!-- Carte info -->
          <div class="bg-white rounded-2xl shadow-sm border border-gray-100 p-6">
            <h3 class="text-lg font-semibold text-gray-800 mb-4 font-display">Nos coordonnées</h3>
            <div class="space-y-4">
              <a href="mailto:uafricas@gmail.com" class="flex items-center gap-3 text-gray-600 hover:text-custom-green transition-colors">
                <div class="w-10 h-10 rounded-full bg-custom-green/10 flex items-center justify-center flex-shrink-0">
                  <font-awesome-icon :icon="['fas', 'envelope']" class="text-custom-green" />
                </div>
                <div>
                  <p class="text-xs text-gray-400">Email</p>
                  <p class="text-sm font-medium">uafricas@gmail.com</p>
                </div>
              </a>
              <div class="flex items-center gap-3 text-gray-600">
                <div class="w-10 h-10 rounded-full bg-custom-chocolat/10 flex items-center justify-center flex-shrink-0">
                  <font-awesome-icon :icon="['fas', 'phone']" class="text-custom-chocolat" />
                </div>
                <div>
                  <p class="text-xs text-gray-400">Téléphone</p>
                  <p class="text-sm font-medium">00 00 00 00 00</p>
                </div>
              </div>
            </div>
          </div>

          <!-- Réseaux sociaux -->
          <div class="bg-white rounded-2xl shadow-sm border border-gray-100 p-6">
            <h3 class="text-lg font-semibold text-gray-800 mb-4 font-display">Suivez-nous</h3>
            <div class="flex gap-3">
              <a
                v-for="reseau in reseauxSociaux"
                :key="reseau.nom"
                href="#"
                :title="reseau.nom"
                class="w-10 h-10 rounded-full flex items-center justify-center transition-colors"
                :class="reseau.classes"
              >
                <font-awesome-icon :icon="['fab', reseau.icone]" class="w-5 h-5" />
              </a>
            </div>
          </div>

          <!-- FAQ rapide -->
          <div class="bg-custom-green/5 rounded-2xl border border-custom-green/20 p-6">
            <h3 class="text-lg font-semibold text-gray-800 mb-2 font-display">Questions fréquentes</h3>
            <p class="text-sm text-gray-600 mb-3">Consultez notre FAQ pour trouver rapidement des réponses.</p>
            <NuxtLink
              to="/a-propos/faq"
              class="inline-flex items-center gap-2 text-custom-green text-sm font-semibold hover:underline"
            >
              Voir la FAQ
              <font-awesome-icon :icon="['fas', 'arrow-right']" class="w-3 h-3" />
            </NuxtLink>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
useHead({
  title: 'Contact - UAfricas',
  meta: [
    { name: 'description', content: 'Contactez l\'équipe UAfricas pour toute question ou proposition de collaboration.' }
  ]
})

const reseauxSociaux = [
  { nom: 'Facebook', icone: 'facebook', classes: 'bg-blue-100 text-blue-600 hover:bg-blue-200' },
  { nom: 'Twitter', icone: 'twitter', classes: 'bg-sky-100 text-sky-500 hover:bg-sky-200' },
  { nom: 'LinkedIn', icone: 'linkedin', classes: 'bg-blue-100 text-blue-700 hover:bg-blue-200' },
  { nom: 'Instagram', icone: 'instagram', classes: 'bg-pink-100 text-pink-600 hover:bg-pink-200' },
  { nom: 'YouTube', icone: 'youtube', classes: 'bg-red-100 text-red-600 hover:bg-red-200' },
]
</script>
