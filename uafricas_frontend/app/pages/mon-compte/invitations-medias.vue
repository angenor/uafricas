<template>
  <NuxtLayout name="africans">
    <template #fil-ariane>
      <AfricansFilAriane
        :segments="[{ libelle: 'Mon compte', vers: '/mon-compte/profil' }, { libelle: 'Invitations médias' }]"
      />
    </template>

    <div class="flex flex-col gap-6">
      <header>
        <h1 class="text-[24px]/[1.3] font-bold text-af-encre">Mes invitations médias</h1>
        <p class="mt-1 text-[14px]/[1.5] text-af-corps">Les propositions de co-détenir une chaîne ou une station qui vous ont été adressées.</p>
      </header>
      <div v-if="acceptationReussie" class="mb-6 rounded-lg bg-green-50 border border-green-200 px-4 py-3 text-sm text-green-900">
        Invitation acceptée.
        <NuxtLink to="/mon-compte/mes-supports" class="font-semibold underline hover:no-underline">
          Voir mes supports
        </NuxtLink>
      </div>

      <div v-if="erreur" class="mb-6 rounded-lg bg-red-50 border border-red-200 px-4 py-3 text-sm text-red-900">
        {{ erreur }}
      </div>

      <div v-if="chargement" class="flex justify-center py-16">
        <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-yellow-400"></div>
      </div>

      <div v-else-if="invitations.length === 0" class="text-center py-16">
        <font-awesome-icon :icon="['fas', 'envelope-open']" class="w-12 h-12 text-gray-300 mb-4" />
        <p class="text-gray-600 mb-1">Aucune invitation reçue.</p>
        <p class="text-sm text-gray-400">
          Un propriétaire de chaîne ou de station peut vous inviter à rejoindre son équipe.
        </p>
      </div>

      <ul v-else class="space-y-4">
        <li
          v-for="invitation in invitations"
          :key="invitation.id"
          class="bg-white rounded-xl shadow-sm border border-gray-100 p-5"
          :class="{ 'opacity-60': invitation.expiree }"
        >
          <div class="flex items-start justify-between gap-4 flex-wrap mb-2">
            <div class="min-w-0">
              <h2 class="font-semibold text-gray-900">
                {{ invitation.support_nom || LIBELLES_TYPE_SUPPORT[invitation.type_support] }}
              </h2>
              <p class="text-xs text-gray-400 mt-0.5">
                {{ LIBELLES_TYPE_SUPPORT[invitation.type_support] }} ·
                invitation du {{ dateFormatee(invitation.created_at) }}
              </p>
            </div>
            <span
              class="shrink-0 rounded-full px-3 py-1 text-xs font-bold uppercase tracking-wide"
              :class="styleStatut(invitation)"
            >
              {{ libelleStatut(invitation) }}
            </span>
          </div>

          <p class="text-sm text-gray-600">
            <span class="font-medium text-gray-900">{{ invitant(invitation) }}</span>
            vous propose le rôle de
            <span class="font-medium text-gray-900">{{ LIBELLES_ROLE_DETENTEUR[invitation.role] }}</span>.
          </p>
          <p class="mt-1 text-xs text-gray-500">
            {{ DESCRIPTIONS_ROLE_DETENTEUR[invitation.role] }}
          </p>

          <!-- Expirée : on l'annonce plutôt que de laisser des boutons que le
               serveur rejetterait de toute façon. -->
          <p v-if="invitation.expiree && invitation.statut === 'en_attente'" class="mt-3 text-sm text-gray-500">
            <font-awesome-icon :icon="['fas', 'clock']" class="w-3.5 h-3.5 mr-1" />
            Cette invitation a expiré le {{ dateFormatee(invitation.expire_at) }} ; il n’est plus possible d’y répondre.
          </p>

          <div v-else-if="estActionnable(invitation)" class="mt-4 flex flex-wrap items-center gap-3">
            <button
              type="button"
              class="cursor-pointer rounded-full bg-custom-green px-5 py-1.5 text-sm font-medium text-white transition-colors hover:bg-custom-green/90 disabled:cursor-not-allowed disabled:opacity-50"
              :disabled="enCours === invitation.id"
              @click="repondre(invitation, 'accepter')"
            >
              Accepter
            </button>
            <button
              type="button"
              class="cursor-pointer rounded-full border border-gray-300 bg-white px-5 py-1.5 text-sm font-medium text-gray-700 transition-colors hover:bg-gray-50 disabled:cursor-not-allowed disabled:opacity-50"
              :disabled="enCours === invitation.id"
              @click="repondre(invitation, 'refuser')"
            >
              Refuser
            </button>
            <span class="text-xs text-gray-400">
              À répondre avant le {{ dateFormatee(invitation.expire_at) }}
            </span>
          </div>

          <p v-else-if="invitation.statut === 'acceptee'" class="mt-3 text-sm">
            <NuxtLink to="/mon-compte/mes-supports" class="text-custom-chocolat font-medium hover:underline">
              Retrouver ce support dans « Mes supports »
            </NuxtLink>
          </p>
        </li>
      </ul>
    </div>

    <template #rail>
      <ComptePanneauNavigation />
    </template>
  </NuxtLayout>
</template>
