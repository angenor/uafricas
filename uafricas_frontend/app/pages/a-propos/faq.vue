<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Hero Section -->
    <div class="relative bg-gradient-to-br from-custom-green via-emerald-600 to-teal-600 overflow-hidden">
      <div class="absolute inset-0 opacity-10">
        <div class="absolute inset-0" style="background-image: url('data:image/svg+xml,%3Csvg width=&quot;60&quot; height=&quot;60&quot; viewBox=&quot;0 0 60 60&quot; xmlns=&quot;http://www.w3.org/2000/svg&quot;%3E%3Cg fill=&quot;none&quot; fill-rule=&quot;evenodd&quot;%3E%3Cg fill=&quot;%23ffffff&quot; fill-opacity=&quot;0.4&quot;%3E%3Cpath d=&quot;M36 34v-4h-2v4h-4v2h4v4h2v-4h4v-2h-4zm0-30V0h-2v4h-4v2h4v4h2V6h4V4h-4zM6 34v-4H4v4H0v2h4v4h2v-4h4v-2H6zM6 4V0H4v4H0v2h4v4h2V6h4V4H6z&quot;/%3E%3C/g%3E%3C/g%3E%3C/svg%3E');" />
      </div>

      <div class="relative max-w-7xl mx-auto px-4 pt-24 pb-16 md:pt-32 md:pb-20">
        <div class="text-center text-white">
          <div class="inline-flex items-center gap-2 bg-white/20 backdrop-blur-xs px-4 py-2 rounded-full text-sm font-medium mb-6">
            <font-awesome-icon :icon="['fas', 'circle-info']" class="w-4 h-4" />
            Aide & Support
          </div>
          <h1 class="text-3xl md:text-5xl font-bold mb-4 font-display">
            Foire Aux Questions
          </h1>
          <p class="text-lg md:text-xl text-white/90 max-w-2xl mx-auto">
            Retrouvez les réponses aux questions les plus fréquemment posées sur UAfricas.
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

    <!-- FAQ Accordion -->
    <section class="max-w-3xl mx-auto px-4 md:px-8 py-12">
      <div class="space-y-3">
        <div
          v-for="(item, index) in faqItems"
          :key="index"
          class="bg-white rounded-xl border border-gray-100 shadow-sm overflow-hidden"
        >
          <button
            class="w-full flex items-center justify-between p-5 text-left hover:bg-gray-50 transition-colors"
            @click="toggleItem(index)"
          >
            <span class="font-semibold text-gray-800 pr-4">{{ item.question }}</span>
            <font-awesome-icon
              :icon="['fas', ouvert === index ? 'chevron-up' : 'chevron-down']"
              class="w-4 h-4 text-custom-green flex-shrink-0 transition-transform"
            />
          </button>
          <div
            v-show="ouvert === index"
            class="px-5 pb-5 text-sm text-gray-600 leading-relaxed border-t border-gray-100 pt-4"
          >
            {{ item.reponse }}
          </div>
        </div>
      </div>

      <!-- CTA Contact -->
      <div class="text-center mt-12 p-8 bg-white rounded-2xl border border-gray-100 shadow-sm">
        <font-awesome-icon :icon="['fas', 'comments']" class="w-8 h-8 text-custom-green mb-3" />
        <h3 class="text-lg font-semibold text-gray-800 mb-2">Vous n'avez pas trouvé votre réponse ?</h3>
        <p class="text-gray-600 text-sm mb-4">N'hésitez pas à nous contacter directement.</p>
        <NuxtLink
          to="/a-propos/contact"
          class="inline-flex items-center gap-2 px-6 py-3 bg-custom-green text-white rounded-full hover:bg-custom-green/90 transition-colors shadow-md"
        >
          <font-awesome-icon :icon="['fas', 'envelope']" />
          Nous contacter
        </NuxtLink>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
useHead({
  title: 'FAQ - UAfricas',
  meta: [
    { name: 'description', content: 'Foire aux questions - Trouvez les réponses à vos questions sur UAfricas.' }
  ]
})

const ouvert = ref<number | null>(null)

const toggleItem = (index: number) => {
  ouvert.value = ouvert.value === index ? null : index
}

const faqItems = [
  {
    question: "Qu'est-ce que UAfricas ?",
    reponse: "UAfricas (United Africa for Sustainable Development) est un réseau international qui réunit les africain(e)s et afro-descendant(e)s autour de la promotion des valeurs culturelles, socio-économiques et scientifiques de l'Afrique. Le réseau vise un développement durable du continent à travers l'union des populations africaines et l'expertise de la diaspora."
  },
  {
    question: "Comment puis-je m'inscrire sur la plateforme ?",
    reponse: "Vous pouvez vous inscrire en cliquant sur le bouton « Se connecter » dans la barre de navigation, puis en choisissant « Créer un compte ». Remplissez le formulaire d'inscription avec vos informations personnelles. Un email de vérification vous sera envoyé pour activer votre compte."
  },
  {
    question: "Comment devenir partenaire de UAfricas ?",
    reponse: "Pour devenir partenaire, rendez-vous sur la page « Devenir Partenaire » accessible depuis le menu. Remplissez le formulaire de demande de partenariat en précisant votre organisation, votre motivation et le type de partenariat souhaité. Notre équipe examinera votre demande."
  },
  {
    question: "Qu'est-ce que le programme d'échange sabbatique ?",
    reponse: "Le programme d'échange sabbatique permet aux membres du réseau de participer à des échanges entre pays africains et la diaspora. Il vise à favoriser le partage de compétences, le transfert de savoir-faire et le renforcement des liens entre les communautés africaines à travers le monde."
  },
  {
    question: "Comment trouver un expert sur la plateforme ?",
    reponse: "Accédez à la section « Experts » depuis le menu principal. Vous pouvez rechercher des experts par domaine de compétence, par pays ou par spécialité. Chaque profil d'expert détaille ses qualifications, son expérience et ses domaines d'intervention."
  },
  {
    question: "Comment contribuer à la gouvernance ?",
    reponse: "La section « Gouvernance » vous permet de participer à travers plusieurs outils : FactCheck pour vérifier les informations, IdeaForces pour proposer des idées constructives, et BadHabits pour identifier les mauvaises pratiques à combattre. Chaque contribution est soumise à modération."
  },
  {
    question: "La plateforme est-elle gratuite ?",
    reponse: "Oui, l'inscription et l'accès aux fonctionnalités de base de UAfricas sont entièrement gratuits. La plateforme repose sur le volontariat de ses membres et le soutien de ses partenaires. Certains services spécifiques pourront être proposés à l'avenir."
  },
  {
    question: "Comment contacter l'équipe UAfricas ?",
    reponse: "Vous pouvez nous contacter par email à uafricas@gmail.com ou via notre page de contact. Notre équipe s'efforcera de répondre à votre demande dans les meilleurs délais."
  }
]
</script>
